use crate::libs::file;
use bytes::{BufMut, Bytes, BytesMut};
use hyper::http::HeaderValue;
use hyper::{Body, Response, StatusCode};
use std::collections::HashMap;
use std::str;

pub fn get_mime_type(ext_name: &str) -> &str {
    match ext_name {
        "html" | "htm" | "shtml" => "text/html",
        "css" => "text/css",
        "xml" => "text/xml",
        "gif" => "image/gif",
        "jpeg" | "jpg" => "image/jpeg",
        "js" => "application/x-javascript",
        "atom" => "application/atom+xml",
        "rss" => "application/rss+xml",
        "mml" => "text/mathml",
        "jad" => "text/vnd.sun.j2me.app-descriptor",
        "wml" => "text/vnd.wap.wml",
        "htc" => "text/x-component",
        "png" => "image/png",
        "tif" | "tiff" => "image/tiff",
        "wbmp" => "image/vnd.wap.wbmp",
        "ico" => "image/x-icon",
        "jng" => "image/x-jng",
        "bmp" => "image/x-ms-bmp",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "jar" | "war" | "ear" => "application/java-archive",
        "hqx" => "application/mac-binhex40",
        "doc" => "application/msword",
        "pdf" => "application/pdf",
        "ps" | "eps" | "ai" => "application/postscript",
        "rtf" => "application/rtf",
        "xls" => "application/vnd.ms-excel",
        "ppt" => "application/vnd.ms-powerpoint",
        "wmlc" => "application/vnd.wap.wmlc",
        "kml" => "application/vnd.google-earth.kml+xml",
        "kmz" => "application/vnd.google-earth.kmz",
        "7z" => "application/x-7z-compressed",
        "cco" => "application/x-cocoa",
        "jardiff" => "application/x-java-archive-diff",
        "jnlp" => "application/x-java-jnlp-file",
        "run" => "application/x-makeself",
        "pl" | "pm" => "application/x-perl",
        "prc" | "pdb" => "application/x-pilot",
        "rar" => "application/x-rar-compressed",
        "rpm" => "application/x-redhat-package-manager",
        "sea" => "application/x-sea",
        "swf" => "application/x-shockwave-flash",
        "sit" => "application/x-stuffit",
        "tcl" | "tk" => "application/x-tcl",
        "der" | "pem" | "crt" => "application/x-x509-ca-cert",
        "xpi" => "application/x-xpinstall",
        "xhtml" => "application/xhtml+xml",
        "zip" => "application/zip",
        "bin" | "exe" | "dll" => "application/octet-stream",
        "deb" => "application/octet-stream",
        "dmg" => "application/octet-stream",
        "eot" => "application/octet-stream",
        "iso" | "img" => "application/octet-stream",
        "msi" | "msp" | "msm" => "application/octet-stream",
        "mid" | "midi" | "kar" => "audio/midi",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "ra" => "audio/x-realaudio",
        "3gpp" | "3gp" => "video/3gpp",
        "mpeg" | "mpg" => "video/mpeg",
        "mov" => "video/quicktime",
        "flv" => "video/x-flv",
        "mng" => "video/x-mng",
        "asx" | "asf" => "video/x-ms-asf",
        "wmv" => "video/x-ms-wmv",
        "avi" => "video/x-msvideo",
        "m4v" | "mp4" => "video/mp4",
        "txt" => get_plain_mime_type(),
        _ => get_plain_mime_type(),
    }
}

pub fn get_plain_mime_type() -> &'static str {
    "text/plain"
}

pub fn get_json_mime_type() -> &'static str {
    "application/json"
}

pub fn set_content_type(mut response: Response<Body>, content_type: &str) -> Response<Body> {
    let t = response.headers_mut();
    t.insert("Content-Type", HeaderValue::from_str(content_type).unwrap());
    response
}

pub fn set_status_code(mut response: Response<Body>, code: StatusCode) -> Response<Body> {
    *response.status_mut() = code;
    response
}

pub fn set_json_body(mut response: Response<Body>, data: &str) -> Response<Body> {
    let value: serde_json::Value = serde_json::from_str(data).unwrap();
    let mut buf = BytesMut::new().writer();
    serde_json::to_writer(&mut buf, &value).unwrap();
    *response.body_mut() = Body::from(buf.into_inner().freeze());
    response
}

fn get_empty_response() -> Response<Body> {
    Response::new(Body::empty())
}

pub fn build_method_not_found_error_response() -> Response<Body> {
    let mut response = get_empty_response();
    response = set_status_code(response, StatusCode::NOT_FOUND);
    response = set_content_type(response, get_json_mime_type());
    response = set_json_body(
        response,
        r#"{ "message": "This Method is not supported!" }"#,
    );
    response
}

pub fn get_mime_type_by_name(path: &str) -> &str {
    let ext_name = file::get_ext_name(path);
    get_mime_type(ext_name)
}

pub fn file_response(path: &str, content: Bytes) -> Response<Body> {
    let mime_type = get_mime_type_by_name(path);
    let response = Response::new(Body::from(content));
    let response = set_content_type(response, mime_type);
    println!("{:?}", response);
    response
}

pub fn build_json_message_response(code: StatusCode, message: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    response = set_content_type(response, get_json_mime_type());
    response = set_status_code(response, code);

    let mut data = r#"{"message":""#.to_string();
    data.push_str(message);
    data.push_str("\"}");
    response = set_json_body(response, &data);

    response
}

pub fn invalid_query_response() -> Response<Body> {
    build_json_message_response(StatusCode::BAD_REQUEST, "Invalid query params")
}

pub fn parse_query(query: &str) -> Result<HashMap<&str, &str>, Response<Body>> {
    let mut map = HashMap::new();
    let params = query.split("&");
    for param in params {
        let mut itr = param.split("=");
        let key = match itr.next() {
            Some(key_name) => match map.get(key_name) {
                Some(_) => return Err(invalid_query_response()),
                None => key_name,
            },
            None => return Err(invalid_query_response()),
        };

        match itr.next() {
            Some(value) => {
                map.insert(key, value);
                value
            }
            None => return Err(invalid_query_response()),
        };
        let extra = itr.next();
        match extra {
            Some(_) => return Err(invalid_query_response()),
            _ => {}
        };
    }
    Ok(map)
}
