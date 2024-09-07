CREATE TABLE history
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,  -- 自增主键
    link       TEXT,                               -- link 信息
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 添加时间，默认当前时间
);