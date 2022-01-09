use crate::dir_map::DirMap;
use crate::{compare, dir_map, reset_dur, throw};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Mutex;
use std::time::Instant;
use tauri::api::{dialog, shell};
use tauri::{command, State, Window};

fn parse_output(bytes: Vec<u8>) -> Result<String, String> {
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

fn check_cmd_success(status: &ExitStatus, stderr: Vec<u8>) -> Result<(), String> {
  if !status.success() {
    let stderr = parse_output(stderr)?;
    throw!("tmutil error {}:\n{}", code_to_str(status.code()), stderr);
  }
  Ok(())
}

#[derive(Default)]
pub struct List(pub Mutex<Option<String>>);

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

#[command]
pub async fn load_backups(
  refresh: bool,
  w: Window,
  backups_list_str: State<'_, List>,
) -> Result<Option<String>, String> {
  if !refresh {
    if let Some(s) = &*backups_list_str.0.lock().unwrap() {
      return Ok(Some(s.clone()));
    }
  }

  full_disk_access(w).await?;

  // return Ok(Some(
  //   "/Volumes/Time Machine Backups/Backups.backupdb/my-mac/2021-12-21-133750\n\
  //   /Volumes/Time Machine Backups/Backups.backupdb/my-mac/2021-12-27-193733\n"
  //     .to_string(),
  // ));

  let output = Command::new("tmutil")
    .arg("listbackups")
    .output()
    .expect("Error calling command");
  check_cmd_success(&output.status, output.stderr.clone())?;

  let output_str = parse_output(output.stdout)?;
  let mut s = backups_list_str.0.lock().unwrap();
  *s = Some(output_str.clone());

  Ok(Some(output_str))
}

#[command]
pub async fn compare_backups(old: String, new: String, w: Window) -> Result<DirMap, String> {
  full_disk_access(w).await?;

  let mut anchor = Instant::now();

  let mut cmd = Command::new("tmutil")
    .arg("compare")
    .arg("-X")
    .arg("-s")
    .arg(&old)
    .arg(&new)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Error calling command");

  println!("\u{23f1}  {:.3}ms running tmutil", reset_dur(&mut anchor));

  let mut child_out = BufReader::new(cmd.stdout.as_mut().unwrap());
  let mut lines = Vec::new();

  loop {
    match child_out.read_until(b'\n', &mut lines) {
      Ok(0) => break,
      _ => {}
    };
  }

  let output = cmd.wait_with_output().expect("Failed ot wait on command");
  check_cmd_success(&output.status, output.stderr)?;

  println!("\u{23f1}  {:.3}ms reading output", reset_dur(&mut anchor));

  let comparison = compare::parse_xml(&lines)?;
  println!("{:#?}", comparison.totals);

  println!("\u{23f1}  {:.3}ms parse xml", reset_dur(&mut anchor));

  let dir_map = dir_map::make_map(comparison)?;

  println!("\u{23f1}  {:.3}ms constructing map", reset_dur(&mut anchor));

  Ok(dir_map)
}
