mod libs;

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

    let request = libs::parse_request(&request);
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
                Err(e) => (
                    "HTTP/1.1 404 NOT FOUND",
                    e.to_string(),
                    libs::get_plain_type(),
                ),
                Ok(mut t) => {
                    let mut file_content = String::new();
                    t.read_to_string(&mut file_content).unwrap_or_default();
                    let ext_name = libs::get_ext_name(path);
                    let mime_type = libs::get_mime_type(ext_name);
                    ("HTTP/1.1 200 OK", file_content, mime_type)
                }
            }
        }
        etc => (
            "HTTP/1.1 404 NOT FOUND",
            String::from(format!("Method {} is not supported!", etc)),
            libs::get_plain_type(),
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
