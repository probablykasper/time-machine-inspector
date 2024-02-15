use crate::cmd::LoadedBackupItem;
use crate::{compare, throw};
use serde::Serialize;
use specta::Type;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub type DirContents = HashMap<String, LoadedBackupItem>;

#[derive(Serialize, Clone, Debug, Type, Default)]
pub struct DirMap {
	pub map: HashMap<String, DirContents>,
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

impl DirMap {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}
	pub fn get_or_create_dir(&mut self, path: String) -> &mut DirContents {
		self.map.entry(path).or_insert(HashMap::new())
	}
	pub fn item_entry(&mut self, path: &Path) -> Result<Entry<String, LoadedBackupItem>, String> {
		let dir = get_parent(path)?.to_string_lossy().to_string();
		let basename = get_basename(path)?.to_string_lossy().to_string();

		let dir_contents = self.get_or_create_dir(dir);
		Ok(dir_contents.entry(basename))
	}
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
