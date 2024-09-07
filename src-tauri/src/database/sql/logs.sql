CREATE TABLE logs
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,  -- 自增主键
    category   VARCHAR(100),                       -- 日志分类
    level      VARCHAR(10),                        -- 日志级别
    message    TEXT,                               -- 日志信息
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 创建时间
);