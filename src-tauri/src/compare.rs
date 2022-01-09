use crate::{reset_dur, throw};
use plist::Value;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Cursor};
use std::time::Instant;

#[derive(Serialize, Debug)]
pub struct Comparison {
  pub changes: Vec<Change>,
  pub totals: Totals,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct ComparisonXml {
  #[serde(rename = "Changes")]
  // changes: Vec<Change>,
  changes: Vec<Value>,
  #[serde(rename = "Totals")]
  totals: Totals,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Change {
  Add(Add),
  Update(Update),
  Delete(Delete),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Add {
  #[serde(alias = "AddedItem")]
  pub added_item: Item,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Update {
  #[serde(alias = "Differences")]
  pub differences: Vec<String>,
  #[serde(alias = "NewerItem")]
  pub newer_item: Item,
  #[serde(alias = "OlderItem")]
  pub older_item: Item,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Delete {
  #[serde(alias = "RemovedItem")]
  pub removed_item: Item,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Item {
  #[serde(rename = "Path")]
  pub path: String,
  #[serde(rename = "Size")]
  pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Totals {
  #[serde(alias = "AddedSize")]
  pub added_size: u64,
  #[serde(alias = "ChangedSize")]
  pub changed_size: u64,
  #[serde(alias = "RemovedSize")]
  pub removed_size: u64,
}

pub fn parse_xml(lines: &[u8]) -> Result<Comparison, String> {
  let comparison: ComparisonXml = match plist::from_bytes(lines) {
    Ok(v) => v,
    Err(e) => throw!("Unable to parse response: {}", e),
  };

  let mut changes = Vec::with_capacity(comparison.changes.len());

  let mut anchor = Instant::now();

  for change_dict in &comparison.changes {
    let change: Change = match deserialize_value(&change_dict) {
      Ok(v) => v,
      Err(e) => {
        throw!(
          "Unable to read change: {}\nReceived value: {:#?}",
          e,
          change_dict
        );
      }
    };
    changes.push(change);
  }

  println!("\u{23f1}  {:.3}ms parsing xml", reset_dur(&mut anchor));

  Ok(Comparison {
    changes,
    totals: comparison.totals,
  })
}

fn deserialize_value<T: DeserializeOwned>(value: &Value) -> Result<T, String> {
  let mut buf_writer = BufWriter::new(Vec::new());
  match value.to_writer_binary(&mut buf_writer) {
    Ok(_) => {}
    Err(_) => throw!("Failed serializing change"),
  };
  let bytes = match buf_writer.into_inner() {
    Ok(v) => v,
    Err(_) => throw!("Error change_buf_writer.into_inner"),
  };
  let cursor = Cursor::new(bytes);
  let change: T = match plist::from_reader(cursor) {
    Ok(v) => v,
    Err(e) => throw!("Unable to read item: {}", e.to_string()),
  };
  Ok(change)
}
