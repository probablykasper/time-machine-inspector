use crate::cmd::{check_cmd_success, parse_output};
use crate::dir_map::DirMap;
use std::process::Command;

pub fn listbackups() -> Result<DirMap<()>, String> {
  println!("tmutil listbackups");

  let output = Command::new("tmutil")
    .arg("listbackups")
    .output()
    .expect("Error calling command");
  check_cmd_success(&output.status, output.stderr.clone())?;
  println!("Success listing backups");

  let output_str = parse_output(output.stdout)?;

  if output_str.trim() == "tmutil error 1:\nNo machine directory found for host." {
    return Ok(DirMap::new());
  }

  let mut paths: Vec<String> = output_str
    .split('\n')
    .map(|s| s.to_string())
    .filter(|s| s != "")
    .collect();

  let mut stored_machine_dir = None;
  let mut get_machine_dir = || -> Result<String, String> {
    match &stored_machine_dir {
      None => {
        println!("tmutil machinedirectory");
        let output = Command::new("tmutil")
          .arg("machinedirectory")
          .output()
          .expect("Error calling command");
        check_cmd_success(&output.status, output.stderr.clone())?;
        println!("Success getting machinedirectory");
        let output_str = parse_output(output.stdout)?.trim().to_string();
        stored_machine_dir = Some(output_str.clone());
        Ok(output_str)
      }
      Some(machine_dir) => Ok(machine_dir.clone()),
    }
  };
  for path in &mut paths {
    if path.starts_with("/") {
      continue;
    } else {
      let machine_dir = get_machine_dir()?;
      *path = machine_dir + "/" + &path;
    }
  }
  if let Some(path0) = paths.get(0) {
    if path0.starts_with("/") {}
  }

  let dir_map = DirMap::from_string_paths(paths)?;
  Ok(dir_map)
}
