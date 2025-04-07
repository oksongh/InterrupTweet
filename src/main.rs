mod endpoints {
    pub mod tweets;
    pub mod users;
}

use std::env;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use sqlx::{Row, SqlitePool};
use tokio::net::TcpListener;

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
    use endpoints::*;
    let app = Router::new()
        .route("/", get(hello_tweet))
        .route("/tweet", post(tweets::create_new_tweet))
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
