use std::convert::Infallible;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use sqlx::{Row, SqlitePool};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("hello world")))
}
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    let pool = SqlitePool::connect("sqlite://dev.db").await?;
    sqlx::query("INSERT INTO users (name, at_id) VALUES (?, ?)")
        .bind("Alice")
        .bind("alicedesu")
        .execute(&pool)
        .await?;

    let rows = sqlx::query("SELECt id, name, at_id FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let at_id: String = row.get("at_id");
        println!("{id},{name},{at_id}");
    }

    let make_svc = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("listen on http://{addr}");
    if let Err(e) = server.await {
        eprintln!("error:{e}");
    }
    Ok(())
}
