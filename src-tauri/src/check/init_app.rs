use log::error;
use tauri::PathResolver;
use crate::enums::path::Resource;
use crate::check::check_file::{check_file_path};
use crate::check::{check_folder};
use crate::enums::result::Result;
use crate::check::r#match::{check_vec};

pub fn check_app_conf() -> bool {
    // 检查 app.conf
    let app_conf = match check_file_path(Resource::AppConf.path()) {
        Result::Existed => { true }
        Result::Created => {
            // 检查文件夹 (resources、backend、db)
            let folders_vec = check_folder::check_folder();
            check_vec(folders_vec);
            true
        }
        Result::Failed(_) => {
            std::process::exit(1);
        }
    };
    app_conf
}

pub fn check_log_db() -> bool {
    // 检查 log.db
    let log_db = match check_file_path(Resource::LogDB.path()) {
        Result::Existed => { true }
        Result::Created => { true }
        Result::Failed(_) => {
            false
        }
    };
    log_db
}

pub fn check_history_db() -> bool {
    // 检查 history.db

}
pub fn check_platform_conf() -> bool {
    // 检查 platform.conf

}


pub fn check_backend(os: &str, arch: &str, resources_path: PathResolver) -> bool {

    // 定义 mac os 的资源路径
    let mac_os_backend_folder = Resource::BackendMacOS.path();
    // 定义 windows 的资源路径
    let win_backend_folder = Resource::BackendWin.path();

    match os {
        // 如果是mac os 根据 arch 删除掉部分资源
        "macos" => {
            match arch {
                "arm" => {
                    // 删除 Resources/subconverter/darwin64/*

                    true
                }
                "x86_64" => {
                    // 删除 Resources/subconverter/darwinarm/*

                    true
                }
                _ => {
                    error!("获取架构{}失败", arch);
                    false
                }
            }
        }
        "windows" => {
            // 检查路径是否存在

        }
        "linux" => {
            error!("并没有linux版本");
            false
        }
        _ => {
            error!("获取系统{}失败", os);
            false
        }
    }
}





