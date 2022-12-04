mod libs;

use crate::libs::http::{not_found_error_response, CustomHttpResponse};
use bytes::{BufMut, Bytes, BytesMut};
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn top_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    let method = req.method();

    let custom_http_response = CustomHttpResponse::new();
    let response = match method {
        // &Method::GET => (),
        // &Method::PUT => (),
        // &Method::DELETE => (),
        _ => custom_http_response.not_found_error_response(),
    };
    Ok(response)
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
