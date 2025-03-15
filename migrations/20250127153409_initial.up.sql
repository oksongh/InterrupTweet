-- Add up migration script here

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    at_id TEXT NOT NULL UNIQUE CHECK (LENGTH(at_id) >= 3),
    birthday TEXT
);

CREATE TABLE tweets (
    id TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
