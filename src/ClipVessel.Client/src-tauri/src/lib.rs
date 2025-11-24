mod services;
mod commands;

use std::error::Error;
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, bail};
use tauri::{tray::TrayIconBuilder, menu::{Menu, MenuItem}, Wry, Manager, App, Window, AppHandle};
use tauri::image::Image;
use crate::commands::is_job_running_command::is_job_running;
use crate::services::video_processor_service::VideoProcessorService;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let video_processor_service: Arc<Mutex<VideoProcessorService>> = Arc::new(Mutex::new(VideoProcessorService::new()));

    tauri::Builder::default().manage(video_processor_service.clone())
                             .plugin(tauri_plugin_opener::init())
                             .invoke_handler(tauri::generate_handler![greet, is_job_running])
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
                          .on_menu_event(|app_handle, event| match event.id.as_ref() {
                              VIEW_LOGS_ID => {
                                  if let Err(err) = show_window(&app_handle, MAIN_WINDOW_LABEL) {
                                      eprintln!("Error: {}", err)
                                  }
                              },
                              PAUSE_RESUME_ID => {
                                  if let Ok(mut video_processor_service) = app_handle.state::<Arc<Mutex<VideoProcessorService>>>().lock() {
                                      let new_is_running_value: bool = !video_processor_service.get_is_running();
                                      match video_processor_service.set_is_running(&app_handle, new_is_running_value) {
                                          Ok(_) => println!("Set is_running to {}", new_is_running_value),
                                          Err(err) => eprintln!("Error: {}", err)
                                      }
                                  }
                              },
                              EXIT_ID => {
                                  app_handle.exit(0);
                              },
                              other => {
                                  eprintln!("Error: Unknown menu item of '{}'", other);
                              }
                          })
                          .build(app)?;

    if let Err(err) = hide_window(&app, MAIN_WINDOW_LABEL) {
        eprintln!("Error: {}", err)
    }

    Ok(())
}

fn hide_window(app: &App, window_label: &str) -> anyhow::Result<()> {
    let window: Window = app.get_window(window_label)
                            .ok_or_else(|| anyhow!("The {} window was not found", window_label))?;

    window.hide()
          .map_err(|err| anyhow!("Unable to hide the {} window: {}", window_label, err))
}

fn show_window(app: &AppHandle, window_label: &str) -> anyhow::Result<()> {
    let window: Window = app.get_window(window_label)
                            .ok_or_else(|| anyhow!("The {} window was not found", window_label))?;

    if window.is_visible()? {
        bail!("Cannot show the {} window, as it's already visible", window_label);
    }

    window.show()
          .map_err(|err| anyhow!("Unable to show the {} window: {}", window_label, err))?;
    window.set_focus()
          .map_err(|err| anyhow!("Unable to set focus to the {} window: {}", window_label, err))
}
