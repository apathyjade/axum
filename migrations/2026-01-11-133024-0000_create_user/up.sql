-- 创建 user 表

CREATE TABLE "users" (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    password VARCHAR(128) NOT NULL,
    email VARCHAR(128),
    phone VARCHAR(16),
    real_name VARCHAR(64),
    status INTEGER NOT NULL,
    created_time TIMESTAMP NOT NULL,
    updated_time TIMESTAMP NOT NULL
);