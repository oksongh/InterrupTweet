-- Add up migration script here

CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    handle_name TEXT NOT NULL,
    at_id TEXT NOT NULL UNIQUE CHECK (LENGTH(at_id) >= 3),
    birthday TEXT
);

-- CREATE TABLE followees(
--     user_id INTEGER,
--     followee_user_id INTEGER,
--     FOREIGN KEY (user_id) REFERENCES users(user_id),
--     FOREIGN KEY (followee_user_id) REFERENCES users(user_id)
-- );

CREATE TABLE tweets (
    tweet_id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    replay_to INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (replay_to) REFERENCES tweets(tweet_id)
);
