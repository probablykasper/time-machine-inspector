use crate::cmd::{check_cmd_success, parse_output};
use regex::Regex;
use serde::Serialize;
use specta::Type;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Clone, Type, Default)]
pub struct Destinations {
  /// ID -> Destination
  pub destinations: Option<HashMap<String, Destination>>,
}
impl Destinations {
  pub fn get_destination<'a>(&'a mut self, id: &str) -> Result<&'a mut Destination, String> {
    let paths = self
      .destinations
      .as_mut()
      .ok_or("Destinations not loaded")?;
    let destination = paths
      .get_mut(id)
      .ok_or(format!("Destination not found: {}", id));
    destination
  }
}

#[derive(Serialize, Clone, Type)]
pub struct Destination {
  /// Backups, if loaded
  pub backups: Option<Vec<Backup>>,
  pub mount_point: String,
}
impl Destination {
  pub fn load_backups_list<'a>(&'a mut self) -> Result<&Vec<Backup>, String> {
    let backups = listbackups(&self.mount_point)?;
    self.backups = Some(backups);
    Ok(self.backups.as_ref().unwrap())
  }
}

#[derive(Serialize, Clone, Type, Debug)]
pub struct Backup {
  pub path: String,
  pub name: String,
}

fn listbackups(mount_point: &str) -> Result<Vec<Backup>, String> {
  println!("tmutil listbackups");

  let output = Command::new("tmutil")
    .arg("listbackups")
    .arg("-d")
    .arg(mount_point)
    .output()
    .expect("Error calling command");
  check_cmd_success(&output.status, output.stderr.clone())?;
  println!("Success listing backups");

  let output_str = parse_output(output.stdout)?;
  println!("{output_str}");

  let mut paths: Vec<_> = output_str
    .trim()
    .split('\n')
    .map(|s| s.to_string())
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

  let backups = paths
    .into_iter()
    .map(|path| Backup {
      name: name_from_path(&path),
      path,
    })
    .collect();
  Ok(backups)
}

fn name_from_path(path: &str) -> String {
  let parts: Vec<_> = PathBuf::from(path)
    .components()
    .filter_map(|component| match component {
      std::path::Component::Normal(name) => Some(name.to_string_lossy().to_string()),
      std::path::Component::RootDir => Some("/".to_string()),
      _ => panic!("Unexpected path component"),
    })
    .collect();

  // Examples:
  // /Volumes/Time Machine Backups/Backups.backupdb/computer-name/2022-08-09-032130
  // /Volumes/.timemachine/C5DA5F96-328F-40A3-9FE6-DB1CA872E766/2023-05-17-123613.backup/2023-05-17-123613.backup

  let last = parts.last().cloned().unwrap_or_default();

  let catalina = Regex::new(r"^\d{4}\-\d{2}\-\d{2}\-\d{6}$").unwrap();
  if catalina.is_match(&last) {
    return last;
  }

  let ventura = Regex::new(r"^\d{4}\-\d{2}\-\d{2}\-\d{6}.backup$").unwrap();
  if ventura.is_match(&last) {
    return last.trim_end_matches(".backup").to_string();
  }

  // fallback
  if parts.len() >= 3 && parts[0] == "/" && parts[1] == "Volumes" {
    let mut parts = parts;
    parts.drain(0..3); // remove /Volumes/<Volume>/
    return parts.join("/");
  }
  path.to_string()
}

#[test]
fn test_name_from_path() {
  assert_eq!(
    name_from_path(
      "/Volumes/Time Machine Backups/Backups.backupdb/computer-name/2022-08-09-032130",
    ),
    "2022-08-09-032130"
  );
  assert_eq!(
      name_from_path(
        "/Volumes/.timemachine/C5DA5A96-328E-40F3-9FD6-DB1AC872F6A6/2022-08-09-032130.backup/2022-08-09-032130.backup",
      ),
      "2022-08-09-032130"
    );
  assert_eq!(name_from_path("/Volumes/Something/else"), "else");
  assert_eq!(name_from_path("/Volumes"), "/Volumes");
}
