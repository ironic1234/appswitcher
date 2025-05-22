use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![launch_app, list_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
