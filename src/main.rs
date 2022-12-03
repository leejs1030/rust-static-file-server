mod libs;

use crate::libs::{file, http};
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut request = [0; 512];
    stream.read(&mut request).unwrap();

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
            match file {
                Err(e) => (http::HttpStatus::NotFound, e.to_string(), http::get_plain_mime_type()),
                Ok(file) => {
                    let file_content = file::read_file(file);
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
    stream.write(response.as_bytes()).unwrap();
}
