CREATE TABLE IF NOT EXISTS users (
    user_id serial PRIMARY KEY,
    nickname VARCHAR (50) UNIQUE NOT NULL,
    avatar VARCHAR (255),
    reputation INT DEFAULT 0 NOT NULL
);