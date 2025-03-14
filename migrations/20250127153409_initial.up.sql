-- Add up migration script here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name CHAR(64) NOT NULL,
    at_id CHAR(20) NOT NULL UNIQUE CHECK (char_length(at_id) >= 3),
    birthday DATE
);

CREATE TABLE tweets (
    id uuid PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
