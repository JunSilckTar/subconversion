pub enum Resource {
    // 应用配置文件
    AppDB,
    // 文件夹
    Resources,
    Backend,
    DB,
    // 文件
    HistoryDB,
    LogDB,
    TempLogDB,
    PlatformConf,
}

impl Resource {
    // 返回枚举值对应的路径
    pub fn path(&self) -> &str {
        match self {
            // 应用配置文件
            Resource::AppDB => "resources/app.db",
            // 文件夹
            Resource::Resources => "resources",
            Resource::Backend => "resources/backend",
            Resource::DB => "resources/db",
            // 文件
            Resource::PlatformConf => "resources/backend/platform.conf",
            Resource::HistoryDB => "resources/db/history.db",
            Resource::LogDB => "resources/db/logs.db",
            Resource::TempLogDB => "resources/db/temp_log.db",
        }
    }
}