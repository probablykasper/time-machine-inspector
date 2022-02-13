#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use tauri::api::{dialog, shell};
use tauri::{
  command, regex, scope, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, Window,
  WindowBuilder, WindowUrl,
};

mod cmd;
mod compare;
mod dir_map;
mod listbackups;

#[command]
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
  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    .manage(cmd::BackupList(Default::default()))
    .manage(cmd::LoadedBackups(Default::default()))
    .invoke_handler(tauri::generate_handler![
      error_popup,
      cmd::load_backup_list,
      cmd::get_backup,
      cmd::backups_info,
    ])
    .create_window("main", WindowUrl::default(), |win, webview| {
      let win = win
        .title("Time Machine Inspector")
        .resizable(true)
        .decorations(true)
        // .transparent(true)
        .always_on_top(false)
        .inner_size(1000.0, 700.0)
        .min_inner_size(600.0, 250.0)
        .skip_taskbar(false)
        .fullscreen(false);
      return (win, webview);
    })
    .setup(|app| {
      let window: tauri::Window = app.handle().get_window("main").unwrap();
      #[cfg(target_os = "macos")]
      {
        use tauri_plugin_vibrancy::Vibrancy;
        window.apply_vibrancy(tauri_plugin_vibrancy::MacOSVibrancy::WindowBackground);
      }
      Ok(())
    })
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone()).into(),
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
          let shell_scope = scope::ShellScope::new(scope::ShellScopeConfig {
            open: Some(regex::Regex::new("^https?://").unwrap()),
            scopes: HashMap::new(),
          });
          let link = "https://github.com/probablykasper/time-machine-inspector".to_string();
          shell::open(&shell_scope, link, None).unwrap();
        }
        _ => {}
      }
    })
    .run(ctx)
    .expect("error while running tauri application");
}
