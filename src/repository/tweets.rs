use std::str::FromStr;

use sqlx::SqlitePool;

use crate::{
    domain::{self, tweets::TweetId, users::UserId},
    usecases::fatal_error::FatalError,
};

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
struct TweetRow {
    tweet_id: String,
    user_id: String,
    content: String,
    replay_to: Option<String>,
    created_at: String,
}
impl TweetRow {
    fn to_domain(self) -> domain::tweets::Tweet {
        domain::tweets::Tweet::new(
            TweetId::from_str(&self.tweet_id).unwrap(),
            UserId::from_str(&self.user_id).unwrap(),
            self.content,
            self.replay_to.map(|id| TweetId::from_str(&id).unwrap()),
            chrono::DateTime::parse_from_rfc3339(&self.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        )
    }
}


    Ok(exists)
}

pub async fn find_tweet_by_user_id(
    user_id: &UserId,
    pool: &SqlitePool,
) -> Result<Vec<domain::tweets::Tweet>, FatalError> {
    let user_id = user_id.value().to_string();

    let tweets = sqlx::query_as!(
        TweetRow,
        r#"
        SELECT tweet_id, user_id, content, replay_to, created_at
        FROM tweets
        WHERE user_id = ?"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    let tweets = tweets
        .into_iter()
        .map(|row| row.to_domain())
        .collect::<Vec<_>>();
    Ok(tweets)
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
