// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod check;
mod tool;
mod enums;
mod database;

use log::{info};
use check::init_app;
use crate::tool::open_url::open_url;
use crate::tool::platform::get_platform_with_arch;


fn main() {
    // 初始化 logger
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {

            // 获取 app 的资源路径
            let resources_path = app.path_resolver();

            // 检查 db 文件夹
            if init_app::check_log_db() { info!("check successful!") }

            // 检查 log.db

            // 获取 os 信息
            let (os, arch) = get_platform_with_arch();

            // 检查资源路径是否存在
            if init_app::check_backend(os, arch, resources_path) { info!("check_backend successful!") }

            // 调用 `init_app::check()` 函数来检查应用的其他资源文件。 如果返回 `true`，则打印 "check successful!"。

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_url])
        .run(tauri::generate_context!())
        .expect("error while running application");
}



