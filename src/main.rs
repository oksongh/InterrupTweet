mod endpoints {
    pub mod tweets;
    pub mod users;
}
mod repository;
mod usecases;

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

    use endpoints::*;
    let app = Router::new()
        .route("/", get(hello_tweet))
        .merge(tweets::app())
        .merge(users::app())
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
