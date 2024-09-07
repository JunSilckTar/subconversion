
use crate::tool::open_url;

pub fn build() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_url::open_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}