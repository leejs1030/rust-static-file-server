mod libs;

use crate::libs::{file, http};
use tokio::fs::File;
use std::io::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async {
            handle_connection(stream).await
        });
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

    println!("\n\nrequest header: {:#?}", header);
    println!("\n\nrequest body: {:#?}", body);

    let (status_line, contents, mime_type) = match method {
        "GET" => {
            let file = File::open("./files".to_owned() + path);
            match file.await {
                Err(e) => (
                    http::HttpStatus::NotFound,
                    e.to_string(),
                    http::get_plain_mime_type(),
                ),
                Ok(file) => {
                    let file_content = file::read_file(file).await;
                    let ext_name = file::get_ext_name(path);
                    let mime_type = http::get_mime_type(ext_name);
                    (http::HttpStatus::Ok, file_content, mime_type)
                }
            }
        }
        etc => (
            http::HttpStatus::NotFound,
            String::from(format!("Method {} is not supported!", etc)),
            http::get_plain_mime_type(),
        ),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        mime_type,
        contents
    );
    stream.write(response.as_bytes()).await.unwrap_or(-1)
}
