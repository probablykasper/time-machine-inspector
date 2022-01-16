use crate::cmd::{check_cmd_success, parse_output};
use crate::dir_map::DirMap;
use std::process::Command;

pub fn listbackups() -> Result<DirMap<()>, String> {
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

  let paths = output_str
    .split('\n')
    .map(|s| s.to_string())
    .filter(|s| s != "")
    .collect();

  let dir_map = DirMap::from_string_paths(paths)?;
  Ok(dir_map)
}
