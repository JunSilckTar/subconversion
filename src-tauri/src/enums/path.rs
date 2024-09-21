pub enum Resource {
    // 应用配置文件
    AppConf,
    backend,
    DB,
    // 文件
    HistoryDB,
    LogDB,
}

impl Resource {
    // 返回枚举值对应的路径
    pub fn path(&self) -> &str {
        match self {
            // conf文件
            Resource::AppConf => "app.conf",
            // 文件夹
            Resource::backend => "subconverter",
            Resource::DB => "db",
            // db文件
            Resource::HistoryDB => "db/history.db",
            Resource::LogDB => "db/logs.db",
        }
    }
}