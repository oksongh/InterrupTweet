use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct NewTweet {
    user_id: i64,
    content: String,
}

pub async fn create_new_tweet(
    State(pool): State<SqlitePool>,
    Json(NewTweet { content, user_id }): Json<NewTweet>,
) -> Json<String> {
    let result = sqlx::query!(
        "INSERT INTO tweets (user_id, content) VALUES (?, ?)",
        user_id,
        content
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json("Tweet created successfully".to_string()),
        Err(_) => Json("Failed to tweet".to_string()),
    }
}
#[derive(serde::Serialize)]
pub struct Tweet {
    id: i64,
    user_id: i64,
    content: String,
}

pub async fn get_tweets(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Tweet,
        "SELECT id, user_id, content FROM tweets WHERE user_id = ?",
        user_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(tweets) => Json(tweets).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use sqlx::{migrate::Migrator, SqlitePool};
    use std::env;

    #[cfg(test)]
    static MIGRATOR: Lazy<String> = Lazy::new(|| {
        dotenvy::dotenv().ok();
        env::var("MIGRATIONS_DIR").expect("MIGRATIONS_PATH must be set")
    });

    #[tokio::test]
    async fn test_create_new_tweet() {
        use std::path::Path;

        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        let migrator = Migrator::new(Path::new(&*MIGRATOR)).await.unwrap();
        migrator.run(&pool).await.unwrap();

        sqlx::query!(
            "INSERT INTO users (id, name, at_id) VALUES (?, ?, ?)",
            1,
            "TestUser",
            "TestUser",
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_tweet = NewTweet {
            user_id: 1,
            content: "hello".to_string(),
        };

        let response = create_new_tweet(State(pool), Json(new_tweet)).await;

        assert_eq!(response.0, "Tweet created successfully");
    }
}
