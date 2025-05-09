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
