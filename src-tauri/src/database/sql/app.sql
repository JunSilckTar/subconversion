CREATE TABLE app
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,         -- 自增主键
    feature    TEXT    NOT NULL,                          -- 功能
    is_enabled BOOLEAN NOT NULL DEFAULT 0,                -- 是否开启，默认未开启
    updated_at TIMESTAMP        DEFAULT CURRENT_TIMESTAMP -- 更新时间，默认当前时间
);