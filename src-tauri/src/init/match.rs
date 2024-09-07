pub fn match_result(result: crate::enums::result::Result, path: &str) -> Vec<&str> {
    // 创建 需要开启的功能
    let mut open = Vec::new();
    match result {
        crate::enums::result::Result::Existed => {
            println!("{} 不做任何更改!!!", path)
        }
        crate::enums::result::Result::Created => {
            open.push(path)
        }
        crate::enums::result::Result::Failed(e) => {
            println!("{} 功能暂时关闭, {}", path, e)
        }
    }
    open
}


pub fn check_vec(vec: Vec<String>) {
    if !vec.is_empty() {
        for item in vec {
            println!("{} 创建失败, 后续重试", item);
        }
    }
}