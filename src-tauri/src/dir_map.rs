use crate::compare;
use std::collections::HashMap;
use std::path::PathBuf;

pub type DirItem = u64;
pub type DirContent = HashMap<String, DirItem>;
pub type DirMap = HashMap<String, DirContent>;

pub fn make_map(comparison: compare::Comparison) -> Result<DirMap, String> {
  let mut dir_map: DirMap = HashMap::new();
  dir_map.entry("/".into()).or_default();

  for change in comparison.changes {
    let new_item = match change {
      compare::Change::Add(add) => add.added_item,
      compare::Change::Update(update) => update.newer_item,
      compare::Change::Delete(_) => continue,
    };
    let path = PathBuf::from(new_item.path);

    let mut base = path.file_name().expect("path base");
    let mut parent = path.parent().expect("path parent");
    loop {
      let dir_content = dir_map
        .entry(parent.to_string_lossy().to_string())
        .or_insert(HashMap::new());

      let dir_item = dir_content
        .entry(base.to_string_lossy().to_string())
        .or_insert(0);
      *dir_item += new_item.size;

      base = match parent.file_name() {
        Some(v) => v,
        None => break,
      };
      parent = match parent.parent() {
        Some(v) => v,
        None => break,
      };
    }
  }

  Ok(dir_map)
}
