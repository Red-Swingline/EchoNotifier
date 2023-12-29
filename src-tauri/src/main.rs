// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod notifications;
use config::read_config;
use notifications::start_notification_listener;
use std::path::PathBuf;
use tauri::api::path::home_dir;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn config_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(".config/echonotifier/config.json")
}

#[tauri::command]
fn load_apps() -> Result<String, String> {
    // Read and parse the configuration file
    let config_path = config_path();
    let config = read_config(&config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    let mut html = String::new();
    for app in config.apps {
        html += &format!(
            r#"
            <div class="app-card" data-app-name="{}">
                <div class="app-card-header">
                    <h2>{}</h2>
                    <div class="app-card-actions">
                        <button class="edit-button" onclick="editApp(this)">
                            <img src="./assets/square-edit-outline.svg" alt="Edit">
                        </button>
                        <button class="delete-button" onclick="deleteApp('{}')">
                            <img src="./assets/delete-outline.svg" alt="Delete">
                        </button>
                    </div>
                </div>
                <div class="app-card-body">
                    <p>Sound Path: {}</p>
                </div>
            </div>
        "#,
            app.app, app.app, app.app, app.sound_path
        );
    }
    Ok(html)
}
#[tauri::command]
fn edit_app(app_name: String, new_sound_path: String) -> Result<(), String> {
    let config_path = config_path();
    let mut config =
        config::read_config(&config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    config::update_app_sound_path(&mut config, &app_name, &new_sound_path)
        .map_err(|e| e.to_string())?;

    config::write_config(&config, &*config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn select_sound_file() -> Result<String, String> {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

    tauri::api::dialog::FileDialogBuilder::new()
        .add_filter("Audio Files", &["mp3", "wav"])
        .pick_file(move |path| {
            sender
                .try_send(path)
                .expect("Failed to send over async mpsc channel");
        });

    match receiver.recv().await {
        Some(path) => match path {
            Some(p) => Ok(p.to_string_lossy().into_owned()),
            None => Err("No file was selected".into()),
        },
        None => Err("File dialog was closed".into()),
    }
}
#[tauri::command]
fn delete_app(app_name: String) -> Result<(), String> {
    let config_path = config_path();
    let mut config =
        config::read_config(&config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    config::delete_app(&mut config, &app_name).map_err(|e| e.to_string())?;

    config::write_config(&config, &*config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    Ok(())
}

fn setup_notification_listener() {
    let config_path = config_path();
    if let Ok(config) = read_config(&config_path.to_string_lossy()) {
        start_notification_listener(config);
    } else {
        eprintln!("Failed to read or parse config");
    }
}

#[tauri::command]
fn get_add_app_form() -> String {
    let form_html = r#"
        <div id="addAppForm">
            <input type="text" id="newAppName" placeholder="App Name">
            <button id="selectSoundFileButton">Select Sound File</button>
            <span id="selectedSoundFilePath"></span>
            <input type="hidden" id="fullSoundFilePath">
            <div id="buttonContainer">
                <button onclick="addApp()">Add App</button>
                <button class="cancel-button" onclick="hideAddAppForm()">Cancel</button>
            </div>
        </div>
    "#;
    form_html.to_string()
}

#[tauri::command]
fn add_app(app_name: String, sound_path: String) -> Result<(), String> {
    let config_path = config_path();
    let mut config =
        config::read_config(&config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    config.apps.push(config::AppSoundConfig {
        app: app_name,
        sound_path,
    });

    config::write_config(&config, &*config_path.to_string_lossy()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|_app| {
            setup_notification_listener();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_apps,
            edit_app,
            select_sound_file,
            delete_app,
            get_add_app_form,
            add_app,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, e| match e {
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let window = app_handle.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                }
            }
            _ => {}
        });
}
