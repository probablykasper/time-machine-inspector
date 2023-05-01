use crate::dir_map::DirMap;
use crate::{compare, listbackups, throw};
use serde::Serialize;
use specta::Type;
use std::collections::HashMap;
use std::fs::File;
use std::process::ExitStatus;
use std::sync::{Mutex, MutexGuard};
use tauri::api::dialog;
use tauri::{command, State, Window};

pub fn parse_output(bytes: Vec<u8>) -> Result<String, String> {
  match String::from_utf8(bytes) {
    Ok(s) => Ok(s),
    Err(e) => throw!("Unable to parse output: {}", e),
  }
}

fn code_to_str(code: Option<i32>) -> String {
  match code {
    Some(code) => format!("{}", code),
    None => "None".to_string(),
  }
}

pub fn check_cmd_success(status: &ExitStatus, stderr: Vec<u8>) -> Result<(), String> {
  if !status.success() {
    let stderr = parse_output(stderr)?;
    throw!("tmutil error {}:\n{}", code_to_str(status.code()), stderr);
  }
  Ok(())
}

pub async fn full_disk_access(dialog_window: Window) -> Result<(), String> {
  match File::open("/Library/Preferences/com.apple.TimeMachine.plist") {
    Ok(_file) => {}
    Err(e) => match e.kind() {
      std::io::ErrorKind::PermissionDenied => {
        dialog::message(
          Some(&dialog_window),
          "Full Disk Access",
          "Time Machine Utility requires full disk access to interact with Time Machine.\n\
          To grant access:\n\
          \n\
          1. Go to System Preferences > Security & Privacy > Privacy\n\
          2. Select Full Disk Access on the left\n\
          3. Add and enable Time Machine Inspector on the right",
        );

        open::that("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
          .unwrap();

        return Ok(());
      }
      _ => eprintln!("Unable to open Time Machine preferences: {}", e),
    },
  };
  Ok(())
}

#[derive(Default)]
pub struct BackupList(pub Mutex<Option<DirMap<()>>>);

impl BackupList {
  pub fn lock(&self) -> Result<MutexGuard<Option<DirMap<()>>>, String> {
    match self.0.lock() {
      Ok(mutex) => Ok(mutex),
      Err(e) => throw!("Unable to lock backup list: {}", e),
    }
  }
}

#[command]
#[specta::specta]
pub async fn load_backup_list(
  refresh: bool,
  w: Window,
  state: State<'_, BackupList>,
) -> Result<DirMap<()>, String> {
  // get cached backup_list
  if !refresh {
    let backup_list = state.lock()?;
    match &*backup_list {
      Some(list) => return Ok(list.clone()),
      None => {}
    }
  }

  full_disk_access(w).await?;
  let dir_map = listbackups::listbackups()?;
  println!("Listed backups {:#?}", dir_map);
  state.lock()?.replace(dir_map.clone());
  println!("Replaced state");

  Ok(dir_map)
}

#[derive(Serialize, Clone, Type)]
pub struct LoadedBackupItem {
  #[specta(type = u32)] // tauri bigint fix
  pub size: u64,
}

#[derive(Serialize, Clone)]
pub struct LoadedBackup {
  pub old: String,
  pub new: String,
  pub map: DirMap<LoadedBackupItem>,
  pub loading: bool,
}
pub type LoadedBackupsMap = HashMap<(String, String), LoadedBackup>;

#[derive(Default)]
pub struct LoadedBackups(pub Mutex<LoadedBackupsMap>);

impl LoadedBackups {
  pub fn lock(&self) -> Result<MutexGuard<LoadedBackupsMap>, String> {
    match self.0.lock() {
      Ok(mutex) => Ok(mutex),
      Err(e) => throw!("Unable to lock backup list: {}", e),
    }
  }
}

#[derive(Serialize, Clone, Type)]
pub struct BackupInfo {
  pub old: String,
  pub new: String,
  pub loading: bool,
}

#[command]
#[specta::specta]
pub async fn backups_info(state: State<'_, LoadedBackups>) -> Result<Vec<BackupInfo>, String> {
  let map = state.lock()?;
  let info = map.values().map(|b| BackupInfo {
    old: b.old.clone(),
    new: b.new.clone(),
    loading: b.loading,
  });
  Ok(info.collect())
}

async fn do_compare(old: &str, new: &str, w: Window) -> Result<DirMap<LoadedBackupItem>, String> {
  full_disk_access(w).await?;
  Ok(compare::compare(&old, &new)?)
}

#[command]
#[specta::specta]
pub async fn get_backup<'a>(
  old_b: String,
  new_b: String,
  refresh: bool,
  w: Window,
  state: State<'_, LoadedBackups>,
) -> Result<DirMap<LoadedBackupItem>, String> {
  let old_new = (old_b.clone(), new_b.clone());

  // get cached dir_map
  if !refresh {
    let loaded_backups = state.lock()?;
    match loaded_backups.get(&old_new) {
      Some(loaded_backup) => {
        return Ok(loaded_backup.map.clone());
      }
      None => {}
    }
  }

  {
    let mut loaded_backups = state.lock()?;
    match loaded_backups.get_mut(&old_new) {
      Some(loaded_backup) => {
        if (loaded_backup).loading {
          throw!("Already loading backup");
        }
      }
      None => {
        let backup = LoadedBackup {
          old: old_b.clone(),
          new: new_b.clone(),
          map: DirMap::new(),
          loading: true,
        };
        loaded_backups.insert(old_new.clone(), backup);
      }
    }
  }

  match do_compare(&old_b, &new_b, w).await {
    Ok(dir_map) => {
      let mut loaded_backups = state.lock()?;
      let backup = LoadedBackup {
        old: old_b,
        new: new_b,
        map: dir_map.clone(),
        loading: false,
      };
      loaded_backups.insert(old_new, backup);
      return Ok(dir_map);
    }
    Err(e) => {
      let mut loaded_backups = state.lock()?;
      loaded_backups.remove(&old_new);
      return Err(e);
    }
  }
}
