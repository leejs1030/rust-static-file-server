mod libs;

use crate::libs::file::{get_ext_name, read_file, remove_dir, remove_file, write_file};
use crate::libs::http::{
    build_json_message_response, build_method_not_found_error_response, file_response,
    get_json_mime_type, get_mime_type, get_plain_mime_type, set_content_type, set_json_body,
    set_status_code,
};
use bytes::{BufMut, Bytes, BytesMut};
use http_body_util::StreamBody;
use hyper::body::HttpBody;
use hyper::client::ResponseFuture;
use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
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
        Err((code, message)) => build_json_message_response(code, &message),
    }
}

async fn put_file(file_name: &str, body: Body) -> Response<Body> {
    let content = hyper::body::to_bytes(body).await.unwrap();
    if content.len() == 0 {
        return build_json_message_response(StatusCode::BAD_REQUEST, "File should not be empty!");
    }
    let res = write_file(file_name, content).await;
    match res {
        Ok(_) => build_json_message_response(StatusCode::OK, "File is successfully created!"),
        Err(e) => build_json_message_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

async fn delete_file(path: &str, query: &str) -> Response<Body> {
    let invalid_query_response =
        build_json_message_response(StatusCode::BAD_REQUEST, "Invalid query params");
    let mut map = HashMap::new();
    let params = query.split("&");
    let mut is_directory: Option<bool> = None;
    for param in params {
        let mut itr = param.split("=");
        let key = match itr.next() {
            Some(key_name) => match map.get(key_name) {
                Some(_) => return invalid_query_response,
                None => {
                    map.insert(key_name, true);
                    key_name
                }
            },
            None => return invalid_query_response,
        };

        let value = match itr.next() {
            Some(t) => t,
            None => return invalid_query_response,
        };
        let extra = itr.next();
        match extra {
            Some(_) => return invalid_query_response,
            _ => {}
        };

        if key == "type" {
            if value == "file" {
                is_directory = Some(false);
            } else if value == "directory" {
                is_directory = Some(true);
            } else {
                return invalid_query_response;
            }
        }
    }

    let is_directory = match is_directory {
        Some(t) => t,
        None => false,
    };
    let res = match is_directory {
        false => remove_file(path).await,
        true => remove_dir(path).await,
    };
    match res {
        Ok(_) => build_json_message_response(StatusCode::OK, "File is successfully deleted!"),
        Err(e) => build_json_message_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

async fn top_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let t = tokio::spawn(async move {
        let (parts, body) = req.into_parts();
        let path = parts.uri.path();
        let query = parts.uri.query().unwrap_or("");
        let method = parts.method;

        let response = match method {
            Method::GET => get_file(path).await,
            Method::PUT => put_file(path, body).await,
            Method::DELETE => delete_file(path, query).await,
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
