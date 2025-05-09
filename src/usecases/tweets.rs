use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::usecases::fatal_error::FatalError;
use crate::{domain, repository};

#[derive(Serialize, Deserialize, Debug, thiserror::Error)]
pub enum CreateTweetError {
    #[error("user not found")]
    UserNotFound,
    #[error("tweet not found")]
    ReplayToTweetNotFound,
    #[error("content too long")]
    ContentTooLong,
}

#[derive(Deserialize)]
pub struct NewTweet {
    user_id: domain::users::UserId,
    replay_to: Option<domain::tweets::TweetId>,
    content: String,
}
impl NewTweet {
    pub fn new(
        user_id: domain::users::UserId,
        replay_to: Option<domain::tweets::TweetId>,
        content: String,
    ) -> Self {
        Self {
            user_id,
            replay_to,
            content,
        }
    }
    pub fn into(
        Self {
            user_id,
            replay_to,
            content,
        }: Self,
    ) -> domain::tweets::Tweet {
        domain::tweets::Tweet::generate_new_tweet(user_id, content, replay_to)
    }
}

pub async fn create_new_tweet(
    new_tweet: NewTweet,
    State(pool): State<SqlitePool>,
) -> Result<Result<(), CreateTweetError>, FatalError> {
    // validate input
    let NewTweet {
        user_id,
        replay_to,
        content,
    } = &new_tweet;

    if content.len() > 280 {
        return Ok(Err(CreateTweetError::ContentTooLong));
    }

    // check if user exists
    if !repository::users::user_exists(&user_id, &pool).await? {
        return Ok(Err(CreateTweetError::UserNotFound));
    };

    // check if replay_to tweet exists
    if let Some(replay_to) = &replay_to {
        if !repository::tweets::tweet_exists(replay_to, &pool).await? {
            return Ok(Err(CreateTweetError::ReplayToTweetNotFound));
        }
    }
    repository::tweets::save_tweet(&NewTweet::into(new_tweet), &pool).await?;

    Ok(Ok(()))
    // let a = result.map_err(|_| anyhow::anyhow!("Failed to create tweet").into());

    // match result {
    //     Ok(_) => Ok(Ok(())),
    //     Err(_) => Err(anyhow::anyhow!("Failed to create tweet").into()),
    // }
}
