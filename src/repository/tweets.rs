use sqlx::SqlitePool;

use crate::{domain, usecases::fatal_error::FatalError};

pub async fn tweet_exists(
    tweet_id: domain::tweets::TweetId,
    pool: &SqlitePool,
) -> Result<bool, FatalError> {
    let exists = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM tweets WHERE tweet_id = ?)")
        .bind(tweet_id.value().to_string())
        .fetch_one(pool)
        .await?;

    Ok(exists)
}

pub async fn save_tweet(
    tweet: &domain::tweets::Tweet,
    pool: &SqlitePool,
) -> Result<(), FatalError> {
    let domain::tweets::Tweet {
        tweet_id,
        user_id,
        content,
        replay_to,
        created_at,
    } = tweet;

    let tweet_id = tweet_id.value().to_string();
    let user_id = user_id.value().to_string();
    let replay_to = replay_to.as_ref().map(|id| id.value().to_string());

    // RETURNING tweet_id, user_id, content
    sqlx::query!(
        r#"
        INSERT INTO tweets (tweet_id, user_id, content, replay_to, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
        tweet_id,
        user_id,
        content,
        replay_to,
        created_at
    )
    .execute(pool)
    .await?;

    Ok(())
}
