use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::enums::path::Resource;

// 使用 lazy_static 创建全局 HashMap
lazy_static! {
    static ref PATH_TO_SQL: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Resource::AppDB.path(), "database/sql/app.sql");
        m.insert(Resource::TempLogDB.path(), "database/sql/temp_log.sql");
        m.insert(Resource::HistoryDB.path(), "database/sql/history.sql");
        m.insert(Resource::LogDB.path(), "database/sql/logs.sql");
        m
    };
}

// 获取对应的 SQL 文件路径
pub fn get_sql_file(path: &str) -> Option<&'static str> {
    PATH_TO_SQL.get(path).copied()
}

