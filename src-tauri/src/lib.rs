use std::process::Command;

#[tauri::command]
fn launch_app(app_name: String) -> Result<String, String> {
    println!("[DEBUG] launch_app called with app_name = {}", app_name);

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
        .invoke_handler(tauri::generate_handler![launch_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
