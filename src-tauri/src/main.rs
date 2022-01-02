#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::thread;
use tauri::api::{dialog, shell};
use tauri::{
  command, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, Window, WindowBuilder,
  WindowUrl,
};

mod cmd;

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

fn main() {
  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![error_popup, cmd::load_backups])
    .create_window("main", WindowUrl::default(), |win, webview| {
      let win = win
        .title("Time Machine Inspector")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        // .transparent(true)
        .always_on_top(false)
        .inner_size(800.0, 550.0)
        .min_inner_size(400.0, 200.0)
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
          shell::open(
            "https://github.com/probablykasper/time-machine-inspector".to_string(),
            None,
          )
          .unwrap();
        }
        _ => {}
      }
    })
    .run(ctx)
    .expect("error while running tauri application");
}
