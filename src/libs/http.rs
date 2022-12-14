use core::fmt;
use std::collections::HashMap;
use std::str;
use tokio::fs::File;

pub enum HttpStatus {
    NotFound,
    Ok,
}

impl HttpStatus {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND",
            HttpStatus::Ok => "HTTP/1.1 200 OK",
        }
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match self {
            HttpStatus::NotFound => HttpStatus::NotFound.as_str(),
            HttpStatus::Ok => HttpStatus::Ok.as_str(),
        };
        write!(f, "{}", status)
    }
}

fn parse_header(str: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    let mut lines = str.lines();
    let mut uri_info = lines.next().unwrap().split(" ");

    let method = uri_info.next().unwrap().to_string();
    map.insert("method".to_string(), method);
    let url = uri_info.next().unwrap().to_string();
    let pos = url.find("?").unwrap_or(url.len());
    let path = url[0..pos].to_string();
    if pos != url.len() {
        map.insert("query_string".to_string(), url[pos + 1..].to_string());
    }
    map.insert("path".to_string(), path);

    for line in lines {
        let mut key_value = line.split(": ");
        let key = key_value.next().unwrap().to_string();
        let value = key_value.next().unwrap().to_string();
        map.insert(key, value);
    }
    map
}

fn parse_body(str: String) -> HashMap<String, String> {
    let lines = str.lines();
    let mut map: HashMap<String, String> = HashMap::new();
    for line in lines {
        let mut key_value = line.split("=");
        let key = key_value.next().unwrap().to_string();
        let value = key_value.next().unwrap().to_string();
        let value = value.replace("\0", "");
        map.insert(key, value);
    }
    map
}

pub fn parse_request(request: &[u8; 512]) -> HashMap<String, HashMap<String, String>> {
    let tmp = str::from_utf8(request).unwrap().to_string();

    let mut split_request = tmp.split("\r\n\r\n");

    let header_str = split_request.next().unwrap().to_string();
    let body_str = split_request.next().unwrap_or("").to_string();

    let header = parse_header(header_str);

    let content_type = match header.get("Content-Type") {
        Some(t) => t.as_str(),
        _ => "",
    };
    let body = match content_type {
        "application/x-www-form-urlencoded" => parse_body(body_str),
        _ => HashMap::new(),
    };

    let mut request = HashMap::new();
    request.insert("header".to_string(), header);
    request.insert("body".to_string(), body);
    request
}

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

pub fn not_found_error_response(msg: &str) -> String {
    format!(
        "{}\r\nContent-Length: {}\r\nContent-Type:{}\r\n\r\n{}",
        HttpStatus::NotFound,
        msg.len(),
        get_plain_mime_type(),
        msg
    )
}

pub async fn ok_string_response_from_file(file: File, ext_name: &str) -> String {
    let file_content = super::file::read_file(file).await;
    let mime_type = get_mime_type(ext_name);
    format!(
        "{}\r\nContent-Length: {}\r\nContent-Type:{}\r\n\r\n{}",
        HttpStatus::Ok,
        file_content.len(),
        mime_type,
        file_content
    )
}
