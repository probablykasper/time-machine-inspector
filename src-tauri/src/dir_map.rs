use crate::cmd::LoadedBackupItem;
use crate::{compare, throw};
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub type DirContents<I> = HashMap<String, I>;

#[derive(Serialize, Clone, Debug)]
pub struct DirMap<I> {
  pub map: HashMap<String, DirContents<I>>,
}

fn get_parent<'a>(path: &'a Path) -> Result<&'a Path, String> {
  match path.parent() {
    Some(p) => Ok(p),
    None => throw!("No parent of path {}", path.to_string_lossy()),
  }
}
fn get_basename<'a>(path: &'a Path) -> Result<&'a OsStr, String> {
  match path.file_name() {
    Some(p) => Ok(p),
    None => throw!("No base of path {}", path.to_string_lossy()),
  }
}

impl<I> DirMap<I> {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }
  pub fn get_or_create_dir(&mut self, path: String) -> &mut DirContents<I> {
    self.map.entry(path).or_insert(HashMap::new())
  }
  pub fn item_entry(&mut self, path: &Path) -> Result<Entry<String, I>, String> {
    let dir = get_parent(path)?.to_string_lossy().to_string();
    let basename = get_basename(path)?.to_string_lossy().to_string();

    let dir_contents = self.get_or_create_dir(dir);
    Ok(dir_contents.entry(basename))
  }
}

impl DirMap<()> {
  pub fn from_string_paths(str_paths: Vec<String>) -> Result<Self, String> {
    let mut dir_map = DirMap::new();

    for str_path in str_paths {
      let path = PathBuf::from(&str_path);

      for ancestor in path.ancestors() {
        if ancestor == Path::new("/") {
          break;
        }
        dir_map
          .item_entry(ancestor)
          .map_err(|e| format!("Unable to save path \"{}\": {}", str_path, e))?
          .or_insert(());
      }
    }
    Ok(dir_map)
  }
}

impl DirMap<LoadedBackupItem> {
  pub fn from_comparison(comparison: compare::Comparison) -> Result<Self, String> {
    let mut dir_map = DirMap::new();

    for change in comparison.changes {
      let new_item = match change {
        compare::Change::Add(add) => add.added_item,
        compare::Change::Update(update) => update.newer_item,
        compare::Change::Delete(_) => continue,
      };
      let path = PathBuf::from(new_item.path);

      for ancestor in path.ancestors() {
        if ancestor == Path::new("/") {
          break;
        }
        let item = dir_map
          .item_entry(ancestor)?
          .or_insert(LoadedBackupItem { size: 0 });
        item.size += new_item.size;
      }
    }
    Ok(dir_map)
  }
}
