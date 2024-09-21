use log::{error, info};

pub fn match_result(result: crate::enums::result::Result, path: &str) -> Vec<&str> {
    // 创建 需要开启的功能
    let mut open = Vec::new();
    match result {
        crate::enums::result::Result::Existed => {
            info!("{} 不做任何更改!!!", path)
        }
        crate::enums::result::Result::Created => {
            open.push(path);
        }
        crate::enums::result::Result::Failed(e) => {
            error!("{} 功能暂时关闭, {}", path, e)
        }
    }
    open
}


pub fn check_vec(vec: Vec<&str>) {
    if !vec.is_empty() {
        for item in vec {
            info!("{} 创建失败, 后续重试", item);
        }
    }
}