use std::convert::Infallible;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("hello world")))
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let make_svc = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("listen on http://{addr}");
    if let Err(e) = server.await {
        eprintln!("error:{e}");
    }
}
