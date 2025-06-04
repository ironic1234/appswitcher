use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColorConfig {
    pub background: String,
    pub border: String,
    pub text: String,
    pub selected_bg: String,
    pub selected_text: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: "#1e1e2e".to_string(),
            border: "#fab387".to_string(),
            text: "#fab387".to_string(),
            selected_bg: "#fab387".to_string(),
            selected_text: "#1e1e2e".to_string(),
        }
    }
}

#[tauri::command]
fn list_apps() -> Vec<String> {
    let mut apps = Vec::new();

    let app_dirs = vec![
        PathBuf::from("/Applications"),
        PathBuf::from("/System/Applications"),
    ];

    for dir in app_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".app") {
                        // Strip ".app" extension for cleaner display
                        apps.push(name.trim_end_matches(".app").to_string());
                    }
                }
            }
        }
    }

    apps.sort();
    apps
}

#[tauri::command]
fn launch_app(app_name: String) -> Result<String, String> {
    let locations = vec![
        format!("/Applications/{}.app", app_name),
        format!("/System/Applications/{}.app", app_name),
    ];

    for app_path in locations {
        if Path::new(&app_path).exists() {
            let status = Command::new("open")
                .arg(&app_path)
                .status()
                .map_err(|e| format!("Failed to launch {}: {}", app_name, e))?;

            if status.success() {
                return Ok(format!("App '{}' opened", app_name));
            } else {
                return Err(format!("Failed to open '{}': exited with non-zero code", app_name));
            }
        }
    }

    Err(format!("App '{}' not found in standard locations", app_name))
}

#[tauri::command]
fn load_color_config() -> ColorConfig {
    let home_dir = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => return ColorConfig::default(),
    };

    let mut config_path = PathBuf::from(home_dir);
    config_path.push(".config/mofi/colors.toml");

    if let Ok(contents) = fs::read_to_string(&config_path) {
        toml::from_str::<ColorConfig>(&contents).unwrap_or_else(|_| ColorConfig::default())
    } else {
        ColorConfig::default()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![launch_app, list_apps, load_color_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
