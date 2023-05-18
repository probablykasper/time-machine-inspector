use crate::cmd::{check_cmd_success, DestinationsState};
use crate::listbackups::Destination;
use crate::throw;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::process::Command;
use tauri::{command, State};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DestinationInfoXml {
  #[serde(rename = "Destinations")]
  pub destinations: Vec<DestinationXml>,
}

#[derive(Deserialize, Debug, Type)]
#[serde(deny_unknown_fields)]
pub struct DestinationXml {
  #[serde(alias = "Kind")]
  pub kind: String,
  #[serde(alias = "URL")]
  pub url: String,
  #[serde(alias = "Name")]
  pub name: String,
  #[serde(alias = "ID")]
  pub id: String,
  #[serde(alias = "LastDestination")]
  pub last_destination: usize,
  #[serde(alias = "MountPoint")]
  pub mount_point: String,
}

#[derive(Serialize, Debug, Type)]
pub struct DestinationDetail {
  pub id: String,
  pub mount_point: String,
  pub mount_point_name: String,
}

#[command]
#[specta::specta]
pub async fn destinationinfo(
  state: State<'_, DestinationsState>,
) -> Result<Vec<DestinationDetail>, String> {
  let output = Command::new("tmutil")
    .arg("destinationinfo")
    .arg("-X")
    .output()
    .expect("Error calling command");
  check_cmd_success(&output.status, output.stderr.clone())?;
  println!("Success running destinationinfo");

  let output_xml: DestinationInfoXml = match plist::from_bytes(&output.stdout) {
    Ok(v) => v,
    Err(e) => throw!("Unable to parse response: {}", e),
  };

  let mut destinations_map = HashMap::new();
  for destination_xml in &output_xml.destinations {
    destinations_map.insert(
      destination_xml.id.clone(),
      Destination {
        backups: None,
        mount_point: destination_xml.mount_point.clone(),
      },
    );
  }
  state.lock()?.destinations = Some(destinations_map.clone());

  let destinations_details = output_xml
    .destinations
    .into_iter()
    .map(|dest| {
      let mount_point_name = if dest.mount_point.starts_with("/Volumes/") {
        dest.mount_point["/Volumes/".len()..].to_string()
      } else {
        dest.mount_point.clone()
      };
      DestinationDetail {
        id: dest.id,
        mount_point: dest.mount_point,
        mount_point_name,
      }
    })
    .collect();

  Ok(destinations_details)
}
