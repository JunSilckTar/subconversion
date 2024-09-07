
use tauri::Manager;
use tauri::api::shell;

#[tauri::command]
pub fn open_url(app: tauri::AppHandle, url: String) {
    if let Err(e) = shell::open(&app.shell_scope(), url, None) {
        eprintln!("Failed to tool URL: {}", e);
    }
}