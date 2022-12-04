mod libs;

use crate::libs::http::{build_method_not_found_error_response, file_response};
use bytes::{BufMut, Bytes, BytesMut};
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::client::ResponseFuture;
use crate::libs::file::read_file;

async fn get_file(path: &str) -> Response<Body> {
    let content = read_file(path).await;
    match content {
        Ok(content) => file_response(content.into()),
        Err(e) => Response::new(Body::empty()),
    }
}

async fn top_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    let method = req.method();

    let response = match method {
        &Method::GET => get_file(path).await,
        // &Method::PUT => (),
        // &Method::DELETE => (),
        _ => build_method_not_found_error_response(),
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
