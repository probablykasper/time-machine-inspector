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

#[derive(Serialize, Deserialize, Debug, Type)]
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
  pub last_destination: u32, // u32 due to tauri-specta
  #[serde(alias = "MountPoint")]
  pub mount_point: String,
}

#[command]
#[specta::specta]
pub async fn destinationinfo(
  state: State<'_, DestinationsState>,
) -> Result<Vec<DestinationXml>, String> {
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

  Ok(output_xml.destinations)
}
