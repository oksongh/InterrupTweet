use derive_more::{From, FromStr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::users::UserId;

#[derive(Serialize, Deserialize, Debug, From, FromStr)]
#[serde(transparent)]
pub struct TweetId(Uuid);
impl TweetId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
    pub fn value(&self) -> &Uuid {
        &self.0
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
    pub fn new(
        tweet_id: TweetId,
        user_id: UserId,
        content: String,
        replay_to: Option<TweetId>,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            tweet_id,
            user_id,
            content,
            replay_to,
            created_at,
        }
    }

    pub fn generate_new_tweet(
        user_id: UserId,
        content: String,
        replay_to: Option<TweetId>,
    ) -> Self {
        Self {
            tweet_id: TweetId::new(),
            user_id,
            content,
            replay_to,
            created_at: chrono::Utc::now(),
        }
    }
}
