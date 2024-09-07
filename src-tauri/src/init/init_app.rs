use crate::enums::path::Resource;
use crate::init::check_file::{check_file_path};
use crate::init::{check_folder};
use crate::enums::result::Result;
use crate::init::r#match::{check_vec};

pub fn init() -> bool {
    // 检查 app.db
    let app_db = match check_file_path(Resource::AppDB.path()) {
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

    // 检查 log.db
    let log_db = match check_file_path(Resource::LogDB.path()){
        Result::Existed => {true}
        Result::Created => {true}
        Result::Failed(_) => {
            false
        }
    };

    // 检查 history.db

    // 检查 platform.conf

    app_db && log_db
}





