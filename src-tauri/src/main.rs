#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::thread;
use std::time::Instant;
use tauri::api::{dialog, shell};
use tauri::{
	command, AboutMetadata, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, Window,
	WindowBuilder, WindowUrl,
};

mod cmd;
mod compare;
mod destinationinfo;
mod dir_map;
mod listbackups;

#[command]
#[specta::specta]
fn error_popup(msg: String, win: Window) {
	println!("Error: {}", msg);
	thread::spawn(move || {
		dialog::message(Some(&win), "Error", msg);
	});
}

#[macro_export]
macro_rules! throw {
	($($arg:tt)*) => {{
		return Err(format!($($arg)*))
	}};
}

pub fn reset_dur(since: &mut Instant) -> f32 {
	let dur = Instant::now().duration_since(*since).as_nanos() as f32;
	*since = Instant::now();
	dur / 1000.0 / 1000.0
}

fn main() {
	#[cfg(debug_assertions)]
	{
		tauri_specta::ts::export(
			specta::collect_types![
				error_popup,
				cmd::load_backup_list,
				cmd::get_backup,
				cmd::backups_info,
				destinationinfo::destinationinfo,
			],
			"../bindings.ts",
		)
		.unwrap();
		println!("Generated TS types");
	}

	let ctx = tauri::generate_context!();

	tauri::Builder::default()
		.manage(cmd::DestinationsState(Default::default()))
		.manage(cmd::LoadedBackups(Default::default()))
		.invoke_handler(tauri::generate_handler![
			error_popup,
			cmd::load_backup_list,
			cmd::get_backup,
			cmd::backups_info,
			destinationinfo::destinationinfo,
		])
		.setup(|app| {
			let _window = WindowBuilder::new(app, "main", WindowUrl::default())
				.title("Time Machine Inspector")
				.resizable(true)
				.decorations(true)
				.always_on_top(false)
				.inner_size(1000.0, 700.0)
				.min_inner_size(600.0, 250.0)
				.skip_taskbar(false)
				.fullscreen(false)
				.build()
				.expect("Unable to create window");
			Ok(())
		})
		.menu(Menu::with_items([
			#[cfg(target_os = "macos")]
			MenuEntry::Submenu(Submenu::new(
				&ctx.package_info().name,
				Menu::with_items([
					MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default())
						.into(),
					MenuItem::Separator.into(),
					MenuItem::Services.into(),
					MenuItem::Separator.into(),
					MenuItem::Hide.into(),
					MenuItem::HideOthers.into(),
					MenuItem::ShowAll.into(),
					MenuItem::Separator.into(),
					MenuItem::Quit.into(),
				]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"File",
				Menu::with_items([MenuItem::CloseWindow.into()]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"Edit",
				Menu::with_items([
					MenuItem::Undo.into(),
					MenuItem::Redo.into(),
					MenuItem::Separator.into(),
					MenuItem::Cut.into(),
					MenuItem::Copy.into(),
					MenuItem::Paste.into(),
					#[cfg(not(target_os = "macos"))]
					MenuItem::Separator.into(),
					MenuItem::SelectAll.into(),
				]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"View",
				Menu::with_items([MenuItem::EnterFullScreen.into()]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"Window",
				Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"Help",
				Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
			)),
		]))
		.on_menu_event(|event| {
			let event_name = event.menu_item_id();
			match event_name {
				"Learn More" => {
					let link =
						"https://github.com/probablykasper/time-machine-inspector".to_string();
					shell::open(&event.window().shell_scope(), link, None).unwrap();
				}
				_ => {}
			}
		})
		.run(ctx)
		.expect("error while running tauri application");
}
