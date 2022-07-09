use crate::cmd::{check_cmd_success, parse_output};
use crate::dir_map::DirMap;
use std::process::Command;

pub fn listbackups() -> Result<DirMap<()>, String> {
  // return Ok(Some(
  //   "/Volumes/Time Machine Backups/Backups.backupdb/my-mac/2021-12-21-133750\n\
  //   /Volumes/Time Machine Backups/Backups.backupdb/my-mac/2021-12-27-193733\n"
  //     .to_string(),
  // ));

  println!("tmutil listbackups");

  let output = Command::new("tmutil")
    .arg("listbackups")
    .output()
    .expect("Error calling command");
  check_cmd_success(&output.status, output.stderr.clone())?;
  println!("Success listing backups");

  let output_str = parse_output(output.stdout)?;

  let mut paths: Vec<String> = output_str
    .split('\n')
    .map(|s| s.to_string())
    .filter(|s| s != "")
    .collect();

  let mut machine_dir = None;
  for path in &mut paths {
    if path.starts_with("/") {
      continue;
    }
    match &machine_dir {
      None => {
        println!("tmutil machinedirectory");
        let output = Command::new("tmutil")
          .arg("machinedirectory")
          .output()
          .expect("Error calling command");
        check_cmd_success(&output.status, output.stderr.clone())?;
        println!("Success getting machinedirectory");
        let output_str = parse_output(output.stdout)?;
        machine_dir = Some(output_str);
      }
      Some(machine_dir) => {
        *path = machine_dir.to_string() + "/" + &path;
      }
    }
  }
  if let Some(path0) = paths.get(0) {
    if path0.starts_with("/") {}
  }

  let dir_map = DirMap::from_string_paths(paths)?;
  Ok(dir_map)
}
