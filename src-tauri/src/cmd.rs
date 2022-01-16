use crate::dir_map::DirMap;
use crate::{compare, listbackups, throw};
use std::fs::File;
use std::process::ExitStatus;
use std::sync::{Mutex, MutexGuard};
use tauri::api::{dialog, shell};
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
        let link = "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles";
        shell::open(link.to_string(), None).unwrap();

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
pub async fn load_backups(
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
  state.lock()?.replace(dir_map.clone());

  Ok(dir_map)
}

#[command]
pub async fn compare_backups<'a>(
  old: String,
  new: String,
  w: Window,
) -> Result<DirMap<u64>, String> {
  full_disk_access(w).await?;
  let dir_map = compare::compare(&old, &new)?;
  Ok(dir_map)
}
