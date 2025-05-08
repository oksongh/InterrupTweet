use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::users::UserId;

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TweetId(Uuid);
impl TweetId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub tweet_id: TweetId,
    pub user_id: UserId,
    pub content: String,
    pub replay_to: Option<TweetId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Tweet {
    pub fn generate_new_tweet(
        user_id: UserId,
        content: String,
        replay_to: Option<TweetId>,
    ) -> Self {
        Self {
            tweet_id: TweetId(Uuid::now_v7()),
            user_id,
            content,
            replay_to,
            created_at: chrono::Utc::now(),
        }
    }
}
