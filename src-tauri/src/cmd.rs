use crate::throw;
use std::fs::File;
use std::process::Command;
use tauri::api::{dialog, shell};
use tauri::{command, Window};

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

#[command]
pub async fn load_backups(w: Window) -> Result<Option<String>, String> {
  match File::open("/Library/Preferences/com.apple.TimeMachine.plist") {
    Ok(_file) => {}
    Err(e) => match e.kind() {
      std::io::ErrorKind::PermissionDenied => {
        dialog::message(
          Some(&w),
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

        return Ok(None);
      }
      _ => eprintln!("Unable to open Time Machine preferences: {}", e),
    },
  };

  let cmd = Command::new("tmutil")
    .arg("listbackups")
    .output()
    .expect("Error getting backups");

  if !cmd.status.success() {
    let stderr = parse_output(cmd.stderr)?;
    eprintln!(
      "listbackups exited with error code {}. stderr:\n{}",
      code_to_str(cmd.status.code()),
      stderr,
    );
    throw!("{}", stderr);
  }

  Ok(Some(parse_output(cmd.stdout)?))
}
