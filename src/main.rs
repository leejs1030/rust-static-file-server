mod libs;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use libs::root::router as root_router;
use std::convert::Infallible;
use std::net::SocketAddr;

async fn top_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let join_handle = tokio::spawn(async move { root_router::router(req).await });
    join_handle.await.unwrap()
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(top_router)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
