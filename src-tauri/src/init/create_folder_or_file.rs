use std::fs;
use std::io;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use crate::database::create::create_table;

const MAX_RETRIES: u32 = 3; // 最大重试次数
const RETRY_DELAY: Duration = Duration::from_secs(3); // 每次重试的延迟

/// 创建文件夹的方法, 包含重试机制
pub fn create_folder(path: &str) -> io::Result<()> {
    // 参数校验
    if path.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path cannot be empty or blank"));
    }

    let folder_path = Path::new(path);
    let mut attempts = 0;

    while attempts < MAX_RETRIES {
        if !folder_path.exists() {
            println!("Folder {:?} does not exist. Creating...", folder_path);
            match fs::create_dir_all(folder_path) {
                Ok(_) => {
                    println!("Folder {:?} created successfully.", folder_path);
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Failed to create folder {:?}: {}. Retrying...", folder_path, e);
                    attempts += 1;
                    sleep(RETRY_DELAY);
                }
            }
        } else {
            println!("Folder {:?} already exists.", folder_path);
            return Ok(());
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "Failed to create folder after multiple attempts"))
}

/// 创建文件的方法, 包含重试机制
pub fn create_file(path: &str) -> io::Result<()> {

    // 校验
    if path.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path cannot be empty or blank"));
    }

    // 检查路径是否以 .db 结尾
    if path.ends_with(".db") {
        return create_table(path);
    }

    // 定义 path
    let file_path = Path::new(path);

    // 检查文件所在的父目录是否存在
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            println!("Folder {:?} does not exist. Creating...", parent);
            create_folder(parent.to_str().unwrap_or(""))?; // 递归创建文件夹
        }
    }

    // 尝试创建文件
    let mut attempts = 0;
    while attempts < MAX_RETRIES {
        if !file_path.exists() {
            println!("File {:?} does not exist. Creating...", file_path);
            match fs::File::create(file_path) {
                Ok(_) => {
                    println!("File {:?} created successfully.", file_path);
                    return Ok(()); // 成功创建文件，返回成功
                }
                Err(e) => {
                    eprintln!("Failed to create file {:?}: {}. Retrying...", file_path, e);
                    attempts += 1;
                    sleep(RETRY_DELAY);
                }
            }
        } else {
            println!("File {:?} already exists.", file_path);
            return Ok(()); // 文件已经存在，返回成功
        }
    }

    // 如果重试次数已用尽，返回失败
    Err(io::Error::new(io::ErrorKind::Other, "Failed to create file after multiple attempts"))
}


