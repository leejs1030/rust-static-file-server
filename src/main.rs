mod libs;

use crate::libs::file::{get_ext_name, read_file};
use crate::libs::http::{
    build_method_not_found_error_response, file_response, get_json_mime_type, get_mime_type,
    get_plain_mime_type, set_content_type, set_json_body, set_status_code,
};
use bytes::{BufMut, Bytes, BytesMut};
use hyper::client::ResponseFuture;
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::fmt::format;
use std::net::SocketAddr;
use std::thread::JoinHandle;

async fn get_file(path: &str) -> Response<Body> {
    let content = read_file(path).await;
    match content {
        Ok(content) => {
            let mut response = Response::new(Body::from(content));
            let ext_name = get_ext_name(path);
            let mime_type = get_mime_type(ext_name);
            set_content_type(response, mime_type)
        }
        Err(e) => {
            let (code, data) = e;
            let mut response = Response::new(Body::empty());
            response = set_content_type(response, get_json_mime_type());
            response = set_status_code(response, code);
            response = set_json_body(response, &data);
            response
        }
    }
}

async fn top_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let t = tokio::spawn(async move {
        let path = req.uri().path();
        let method = req.method();

        let response = match method {
            &Method::GET => get_file(path).await,
            // &Method::PUT => (),
            // &Method::DELETE => (),
            _ => build_method_not_found_error_response(),
        };
        println!("{:?}", response);
        Ok(response)
    });
    t.await.unwrap()
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
