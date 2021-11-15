-- Add up migration script here
CREATE TABLE IF NOT EXISTS blog (
    blog_id serial PRIMARY KEY,
    content TEXT,
    created_at TIMESTAMP DEFAULT current_timestamp,
    user_id int NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
)