use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[tauri::command]
fn list_apps() -> Vec<String> {
    let apps_dir = PathBuf::from("/Applications");
    let mut apps = Vec::new();

    if let Ok(entries) = fs::read_dir(apps_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".app") {
                    let app_name = name.trim_end_matches(".app").to_string();
                    apps.push(app_name);
                }
            }
        }
    }

    apps.sort();
    apps
}

#[tauri::command]
fn launch_app(app_name: String) -> Result<String, String> {
    let app_path = format!("/Applications/{}.app", app_name);

    let status = Command::new("open")
        .arg(&app_path)
        .status()
        .map_err(|e| format!("Failed to launch app: {}", e))?;

    if status.success() {
        Ok(format!("App {} opened", app_name))
    } else {
        Err(format!("App exited with non-zero code"))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![launch_app, list_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
