use std::path::Path;
use crate::init::create_folder_or_file::create_file;
use crate::enums::result::Result;

pub fn check_file_path(path: &str) -> Result {
    // 先检查路径下的文件是否存在
    if Path::new(&path).exists() {
        return Result::Existed; // 存在 
    }

    // 文件不存在，尝试创建文件
    match create_file(&path) {
        Ok(_) => { Result::Created }
        Err(e) => { Result::Failed(e.to_string()) }
    }
}



