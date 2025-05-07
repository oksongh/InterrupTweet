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
    user_id: i64,
    replay_to: Option<i64>,
    content: String,
}
impl NewTweet {
    pub fn new(user_id: i64, replay_to: Option<i64>, content: String) -> Self {
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
        domain::tweets::Tweet::new(
            0, // TODO: set tweet_id
            user_id,
            content,
            replay_to,
            "".to_string(), // TODO: set created_at
        )
    }
}

pub async fn create_new_tweet(
    NewTweet {
        content,
        user_id,
        replay_to,
    }: NewTweet,
    State(pool): State<SqlitePool>,
) -> Result<Result<(), CreateTweetError>, FatalError> {
    // validate input

    if content.len() > 280 {
        return Ok(Err(CreateTweetError::ContentTooLong));
    }

    // check if user exists
    if !repository::users::user_exists(user_id, &pool).await? {
        return Ok(Err(CreateTweetError::UserNotFound));
    };

    // check if replay_to tweet exists
    if let Some(replay_to) = replay_to {
        if !tweet_exists(replay_to, &pool).await? {
            return Ok(Err(CreateTweetError::ReplayToTweetNotFound));
        }
    }

    println!("aaaab");

    // RETURNING tweet_id, user_id, content
    sqlx::query!(
        r#"
        INSERT INTO tweets (user_id, replay_to, content)
        VALUES (?, ?, ?)
        "#,
        user_id,
        replay_to,
        content
    )
    .execute(&pool)
    .await?;

    println!("aaa");

    Ok(Ok(()))
    // let a = result.map_err(|_| anyhow::anyhow!("Failed to create tweet").into());

    // match result {
    //     Ok(_) => Ok(Ok(())),
    //     Err(_) => Err(anyhow::anyhow!("Failed to create tweet").into()),
    // }
}

pub async fn tweet_exists(tweet_id: i64, pool: &SqlitePool) -> Result<bool, FatalError> {
    let exists = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM tweets WHERE tweet_id = ?)")
        .bind(tweet_id)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}
