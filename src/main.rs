mod libs;

use crate::libs::file::{get_ext_name, read_file, write_file};
use crate::libs::http::{
    build_method_not_found_error_response, file_response, get_json_mime_type, get_mime_type,
    get_plain_mime_type, set_content_type, set_json_body, set_status_code,
};
use bytes::{BufMut, Bytes, BytesMut};
use http_body_util::StreamBody;
use hyper::body::HttpBody;
use hyper::client::ResponseFuture;
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::borrow::{Borrow, BorrowMut};
use std::convert::Infallible;
use std::fmt::format;
use std::io::BufReader;
use std::net::SocketAddr;
use std::ops::Deref;
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

async fn put_file(file_name: &str, body: Body) -> Response<Body> {
    let content = hyper::body::to_bytes(body).await.unwrap();
    if content.len() == 0 {
        let mut response = Response::new(Body::empty());
        response = set_content_type(response, get_json_mime_type());
        response = set_status_code(response, StatusCode::BAD_REQUEST);

        let mut data = r#"{"message":""#.to_string();
        data.push_str("File should not be empty!");
        data.push_str("\"}");
        response = set_json_body(response, &data);

        return response;
    }
    let res = write_file(file_name, content).await;
    match res {
        Ok(_) => {
            let mut response = Response::new(Body::empty());
            response = set_content_type(response, get_json_mime_type());

            let data = r#"{"message": "File succesfully created!"}"#;
            response = set_json_body(response, data);

            response
        }
        Err(e) => {
            let mut response = Response::new(Body::empty());
            response = set_content_type(response, get_json_mime_type());
            response = set_status_code(response, StatusCode::INTERNAL_SERVER_ERROR);

            let mut data = r#"{"message":""#.to_string();
            data.push_str(&e.to_string());
            data.push_str("\"}");
            response = set_json_body(response, &data);

            response
        }
    }
}

async fn top_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let t = tokio::spawn(async move {
        let (parts, body) = req.into_parts();
        let path = parts.uri.path();
        let method = parts.method;

        let response = match method {
            Method::GET => get_file(path).await,
            Method::PUT => put_file(path, body).await,
            // Method::DELETE => (),
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
