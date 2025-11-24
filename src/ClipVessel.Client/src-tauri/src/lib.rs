use std::error::Error;
use tauri::{tray::TrayIconBuilder, menu::{Menu, MenuItem}, Wry, Manager, App};
use tauri::image::Image;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().plugin(tauri_plugin_opener::init())
                             .invoke_handler(tauri::generate_handler![greet])
                             .setup(|app| setup_system_tray_menu_options(app))
                             .run(tauri::generate_context!())
                             .expect("Error: Unexpected error was encountered while running Clip Vessel");
}

fn setup_system_tray_menu_options(app: &mut App) -> Result<(), Box<dyn Error>> {
    const VIEW_LOGS_ID: &str = "view_logs";
    const PAUSE_RESUME_ID: &str = "pause_resume";
    const EXIT_ID: &str = "exit";

    const MAIN_WINDOW_LABEL: &str = "main";

    let view_logs: MenuItem<Wry> = MenuItem::with_id(app, VIEW_LOGS_ID, "View Logs", true, None::<&str>)?;
    let pause_resume: MenuItem<Wry> = MenuItem::with_id(app, PAUSE_RESUME_ID, "Pause/Resume Jobs", true, None::<&str>)?;
    let exit: MenuItem<Wry> = MenuItem::with_id(app, EXIT_ID, "Exit", true, None::<&str>)?;

    let menu: Menu<Wry> = Menu::with_items(app, &[&view_logs, &pause_resume, &exit])?;
    let icon: Image = app.default_window_icon().expect("No icon exists").clone();

    TrayIconBuilder::new().menu(&menu)
                          .icon(icon)
                          .on_menu_event(|app, event| match event.id.as_ref() {
                              VIEW_LOGS_ID => {
                                  if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                                      if let Ok(showWindowResult) = window.show() &&
                                         let Ok(setFocusResult) = window.set_focus() {
                                          println!("Successfully showed the main window and set focus")
                                      }
                                      else {
                                          println!("Error: Unable to show the main window or set focus")
                                      }
                                  }
                                  else {
                                      println!("Error: Unable to find main window to show")
                                  }
                              },
                              PAUSE_RESUME_ID => {
                                  todo!("Still need to add functionality to handle the background job.");
                              },
                              EXIT_ID => {
                                  app.exit(0);
                              },
                              other => {
                                  println!("Error: Unknown menu item of '{}'", other);
                              }
                          })
                          .build(app)?;

    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
        if let Ok(hideWindowResult) = window.hide() {
            println!("Successfully hid the main window on startup")
        }
        else {
            println!("Error: Unable to hide the main window")
        }
    }
    else {
        println!("Error: Unable to find main window to hide");
    }

    Ok(())
}
