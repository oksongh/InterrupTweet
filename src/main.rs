mod endpoints {
    pub mod tweets;
    pub mod users;
}

use std::env;

use axum::{
    routing::{get, post},
    Router,
};

use sqlx::{Row, SqlitePool};
use tokio::net::TcpListener;

async fn hello_tweet() -> String {
    "Hello".to_string()
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
        .route("/tweet/{:id}", get(tweets::get_tweets))
        .route("/user", post(users::create_new_user).get(users::get_users))
        .route("/user/{id}", get(users::get_user_by_id))
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
