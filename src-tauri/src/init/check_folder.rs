use std::path::Path;
use crate::enums::path::Resource;
use crate::init::create_folder_or_file;
use crate::enums::result::Result;
use crate::init::r#match::match_result;

pub fn check_folder() -> Vec<String> {
    // 获取 backend
    let backend_path = Resource::Backend.path();
    // 获取 db
    let db_path = Resource::DB.path();

    // 检查文件夹
    let backend = check_folder_path(backend_path);
    let db = check_folder_path(db_path);

    // 获取需要设置的值
    let backend_vec = match_result(backend, backend_path);
    let db_vec = match_result(db, db_path);

    backend_vec.into_iter()
        .chain(db_vec.into_iter())
        .map(|s| s.to_string())
        .collect()
}

pub fn check_folder_path(path: &str) -> Result {
    // 检查文件夹是否存在
    if Path::new(&path).exists() { return Result::Existed; }
    // 尝试创建文件夹
    let result = create_folder_or_file::create_folder(path);
    match result {
        Ok(_) => { Result::Created }
        Err(e) => { Result::Failed(e.to_string()) }
    }
}


