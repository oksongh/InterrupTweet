use std::{env, path::Path};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{migrate::Migrator, Row, SqlitePool};
use tokio::net::TcpListener;

async fn hello_tweet() -> String {
    "Hello".to_string()
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
