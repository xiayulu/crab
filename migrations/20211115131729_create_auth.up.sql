-- Add up migration script here
CREATE TABLE IF NOT EXISTS auths (
    auth_id serial PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    last_login TIMESTAMP DEFAULT current_timestamp,
    is_active BOOL DEFAULT true,
    user_id INT UNIQUE NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);