use super::controller;
use crate::libs::http;
use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;

pub async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method();

    let response = match *method {
        Method::GET => controller::get_file(req).await,
        Method::PUT => controller::put_file(req).await,
        Method::DELETE => controller::delete_file(req).await,
        _ => http::build_method_not_found_error_response(),
    };
    println!("{:?}", response);
    Ok(response)
}
