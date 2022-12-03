mod libs;

use std::fs::File;
use std::io::prelude::*;
use std::io::Bytes;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let tmp = str::from_utf8(&buffer).unwrap().to_string();

    let mut arr = tmp.split(" ");
    let method = arr.next().unwrap();
    let path = arr.next().unwrap();

    let (status_line, contents, mime_type) = match method {
        "GET" => {
            let mut file = File::open("./files".to_owned() + path);
            match file {
                Err(E) => (
                    "HTTP/1.1 404 NOT FOUND",
                    E.to_string(),
                    libs::get_plain_type(),
                ),
                Ok(mut T) => {
                    let mut file_content = String::new();
                    T.read_to_string(&mut file_content).unwrap_or_default();
                    let ext_name = libs::get_ext_name(path);
                    let mime_type = libs::get_mime_type(ext_name);
                    ("HTTP/1.1 200 OK", file_content, mime_type)
                }
            }
        }
        etc => (
            "HTTP/1.1 404 NOT FOUND",
            String::from(format!("Cannot GET {}", etc)),
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
