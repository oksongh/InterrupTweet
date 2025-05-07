use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub tweet_id: i64,
    pub user_id: i64,
    pub content: String,
    pub replay_to: Option<i64>,
    pub created_at: String,
}

impl Tweet {
    pub fn new(
        tweet_id: i64,
        user_id: i64,
        content: String,
        replay_to: Option<i64>,
        created_at: String,
    ) -> Self {
        Self {
            tweet_id,
            user_id,
            content,
            replay_to,
            created_at,
        }
    }
}
