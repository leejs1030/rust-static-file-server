use super::service;
use crate::libs::http;
use hyper::{Body, Request, Response, StatusCode};

pub async fn get_file(req: Request<Body>) -> Response<Body> {
    let path = req.uri().path();

    let content = service::read_file_by_path(path).await;
    println!("{:?}", content);
    match content {
        Ok(content) => http::file_response(path, content),
        Err((code, message)) => http::build_json_message_response(code, &message),
    }
}

pub async fn put_file(req: Request<Body>) -> Response<Body> {
    let (parts, body) = req.into_parts();
    let path = parts.uri.path();

    let content = hyper::body::to_bytes(body).await.unwrap();
    if content.len() == 0 {
        return http::build_json_message_response(StatusCode::BAD_REQUEST, "File should not be empty!");
    }
    let res = service::write_file(path, content).await;
    match res {
        Ok(_) => http::build_json_message_response(StatusCode::OK, "File is successfully created!"),
        Err(e) => http::build_json_message_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

pub async fn delete_file(req: Request<Body>) -> Response<Body> {
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    let query_map = http::parse_query(query);
    let query_map = match query_map {
        Err(e) => return e,
        Ok(map) => map,
    };

    let res = service::delete_by_path_and_query(path, &query_map).await;
    match res {
        Ok(_) => http::build_json_message_response(StatusCode::OK, "File is successfully deleted!"),
        Err(e) => http::build_json_message_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}
