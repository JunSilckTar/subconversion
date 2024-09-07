use crate::database::get_sql_file::get_sql_file;
use rusqlite::Connection;
use std::{fs, io};


pub fn create_table(path: &str) -> io::Result<()> {
    // 依传递过来的路径寻找到对应的 SQL 文件路径
    let sql_file_path = match get_sql_file(path) {
        Some(path) if !path.is_empty() => path,
        _ => return Err(io::Error::new(io::ErrorKind::NotFound, "SQL file path not found or is empty.")),
    };

    // 依照路径创建链接
    let conn = Connection::open(path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // 应用 SQL 文件内容到数据库
    apply_sql_file(sql_file_path, &conn)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}

fn apply_sql_file(sql_file_path: &str, conn: &Connection) -> Result<(), rusqlite::Error> {
    // 读取 SQL 文件的内容
    let sql_content = fs::read_to_string(sql_file_path)
        .map_err(|e| rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(e.raw_os_error().unwrap_or(-1)), None))?;

    // 执行 SQL 内容
    conn.execute_batch(&sql_content)?;

    Ok(())
}