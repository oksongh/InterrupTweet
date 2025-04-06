use std::{env, path::Path};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{migrate::Migrator, Row, SqlitePool};
use tokio::net::TcpListener;

#[cfg(test)]
use once_cell::sync::Lazy;

async fn hello_tweet() -> String {
    "Hello".to_string()
}
#[derive(Deserialize, Debug)]
struct NewUser {
    name: String,
    at_id: String,
    birthday: Option<String>,
}

#[derive(Serialize, Debug)]
struct User {
    id: i64,
    name: String,
    at_id: String,
    birthday: Option<String>,
}
// crud
// c : succ :(statuscode::created, json<string>) ,fail(statuscode::bad_request, json<string>)
// r : (statuscode::, json<array<user>>) ,fail(statuscode::not_found, json<string>)
// u : (statuscode, json<string>)
// d : (statuscode, json<string>)
async fn create_new_user(
    State(pool): State<SqlitePool>,
    Json(NewUser {
        name,
        at_id,
        birthday,
    }): Json<NewUser>,
) -> impl IntoResponse {
    // validate input
    if at_id.len() < 3 {
        return (
            StatusCode::BAD_REQUEST,
            Json("at_id should be at least 3 characters long".to_string()),
        )
            .into_response();
    }

    if let Some(birthday) = &birthday {
        if NaiveDate::parse_from_str(birthday, "%Y-%m-%d").is_err() {
            return (
                StatusCode::BAD_REQUEST,
                Json("birthday should be in format YYYY-MM-DD".to_string()),
            )
                .into_response();
        }
    }

    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, at_id, birthday)
        VALUES (?,?,?)
        RETURNING id,name, at_id, birthday
        "#,
        name,
        at_id,
        birthday
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json("failed to create user".to_string()),
        )
            .into_response(),
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn test_create_new_user_birthday_some(pool: SqlitePool) {
    let new_user = NewUser {
        name: "TestUser".to_string(),
        at_id: "test_user".to_string(),
        birthday: Some("2023-01-01".to_string()),
    };
    let response = create_new_user(State(pool.clone()), Json(new_user)).await;
    assert_eq!(response.into_response().status(), StatusCode::CREATED);
}

#[sqlx::test(migrations = "./migrations")]
async fn test_create_new_user_birthday_none(pool: SqlitePool) {
    let new_user = NewUser {
        name: "TestUser".to_string(),
        at_id: "test_user".to_string(),
        birthday: None,
    };
    let response = create_new_user(State(pool.clone()), Json(new_user)).await;
    assert_eq!(response.into_response().status(), StatusCode::CREATED);
}
#[sqlx::test(migrations = "./migrations")]
async fn test_create_new_user_birthday_invalid(pool: SqlitePool) {
    let new_user = NewUser {
        name: "TestUser".to_string(),
        at_id: "test_user".to_string(),
        birthday: Some("2023-13-01".to_string()),
    };
    let response = create_new_user(State(pool.clone()), Json(new_user)).await;
    assert_eq!(response.into_response().status(), StatusCode::BAD_REQUEST);
}

#[derive(Deserialize)]
struct NewTweet {
    user_id: i64,
    content: String,
}

async fn create_new_tweet(
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

#[cfg(test)]
static MIGRATOR: Lazy<String> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    env::var("MIGRATIONS_DIR").expect("MIGRATIONS_PATH must be set")
});

#[tokio::test]
async fn test_create_new_tweet() {
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: SqlitePool = SqlitePool::connect(&db_url).await?;

    let rows = sqlx::query("SELECt id, name, at_id FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let at_id: String = row.get("at_id");
        println!("{id},{name},{at_id}");
    }

    let app = Router::new()
        .route("/", get(hello_tweet))
        .route("/tweet", post(create_new_tweet))
        .route("/user", post(create_new_user))
        .with_state(pool);

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();
    let listener = TcpListener::bind(addr).await?;
    let server = axum::serve(listener, app);

    println!("listen on http://{addr}");
    if let Err(e) = server.await {
        eprintln!("error:{e}");
    }

    Ok(())
}
