use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use sqlx::{Pool, Sqlite, SqlitePool};

use crate::usecases;
use crate::usecases::common::FatalError;

impl IntoResponse for FatalError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.0.to_string())).into_response()
    }
}

impl IntoResponse for usecases::tweets::CreateTweetError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(self.to_string())).into_response()
    }
}

// TODO: -> Result<Result<Json<String>, impl IntoResponse>, ~~anyhow::Error~~ custom error>>
async fn post_tweet(
    State(pool): State<SqlitePool>,
    Json(new_tweet): Json<usecases::tweets::NewTweet>,
) -> Result<impl IntoResponse, FatalError> {
    println!("post_tweet");

    match usecases::tweets::create_new_tweet(new_tweet, State(pool)).await? {
        Ok(_) => Ok(Json("Tweet created successfully".to_string()).into_response()),
        Err(e) => Ok(e.into_response()),
    }
}

#[derive(serde::Serialize)]
struct Tweet {
    tweet_id: i64,
    user_id: i64,
    content: String,
}

async fn get_tweets(State(pool): State<SqlitePool>, Path(user_id): Path<i64>) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Tweet,
        "SELECT tweet_id, user_id, content FROM tweets WHERE user_id = ?",
        user_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(tweets) => Json(tweets).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub fn app() -> Router<Pool<Sqlite>> {
    let api_route = Router::new()
        .route("/", post(post_tweet))
        .route("/{:id}", get(get_tweets));

    Router::new().nest("/tweet", api_route)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use once_cell::sync::Lazy;
    use sqlx::{migrate::Migrator, SqlitePool};
    use std::env;
    use tower::ServiceExt;

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
            "INSERT INTO users (user_id, handle_name, at_id) VALUES (?, ?, ?)",
            1,
            "TestUser",
            "TestUser",
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_tweet = usecases::tweets::NewTweet::new(1, None, "hello".to_string());

        let response = post_tweet(State(pool), Json(new_tweet))
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }

    // #[sqlx::test(migrations = "./migrations")]
    // async fn test_reply_to_tweet(pool: SqlitePool) {
    //     let app = app().with_state(pool);
    //     let req = Request::builder()
    //         .method(Method::POST)
    //         .uri("/tweet")
    //         .header("content-type", "application/json")
    //         .body(Body::from(
    //             serde_json::to_string(&serde_json::json!({
    //                 "user_id": 1,
    //                 "replay_to": 2,
    //                 "content": "hello",
    //             }))
    //             .unwrap(),
    //         ))
    //         .unwrap();

    //     let response = app.oneshot(req).await.unwrap();
    //     assert_eq!(response.status(), StatusCode::OK);
    // }
}
