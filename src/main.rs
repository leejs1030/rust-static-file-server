mod libs;

use crate::libs::{file, http};
use std::io::prelude::*;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async { handle_connection(stream).await });
    }
}

async fn handle_connection(mut stream: TcpStream) -> usize {
    let mut request = [0; 512];
    stream.read(&mut request).await.unwrap();

    let request = http::parse_request(&request);
    let header = request.get("header").unwrap();
    let body = request.get("body").unwrap();
    let method = header.get("method").unwrap().as_str();
    let path = header.get("path").unwrap().as_str();
    let ext_name = file::get_ext_name(path);

    let response = match method {
        "GET" => {
            let file = File::open("./files".to_owned() + path).await;
            match file {
                Err(e) => http::not_found_error_response(&e.to_string()),
                Ok(file) => http::ok_string_response_from_file(file, ext_name).await,
            }
        }
        etc => http::not_found_error_response(format!("Method {} is not supported!", etc).as_str()),
    };
    stream.write(response.as_bytes()).await.unwrap_or(0)
}
