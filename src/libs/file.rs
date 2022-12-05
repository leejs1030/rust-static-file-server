use crate::StatusCode;
use bytes::Bytes;
use std::io::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub fn get_ext_name(file_name: &str) -> &str {
    let mut arr = file_name.rsplit(".");
    arr.next().unwrap_or("_")
}

pub async fn read_file(file_name: &str) -> Result<Vec<u8>, (StatusCode, String)> {
    let path = "./files".to_owned() + file_name;
    let file = tokio::fs::read(path).await;
    match file {
        Ok(contents) => Ok(contents),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Err((
                StatusCode::NOT_FOUND,
                String::from(r#"{"message": "File does not exist!"}"#),
            )),
            _ => {
                let mut data = String::from(r#"{"message": "Internal Server Error""#);
                let message = e.to_string();
                data.push_str(&format!(r#", "detail": "{}""#, &message));
                data.push_str("}");
                Err((StatusCode::INTERNAL_SERVER_ERROR, data))
            }
        },
    }
}

pub async fn write_file(file_name: &str, content: Bytes) -> tokio::io::Result<()> {
    let path = "./files".to_owned() + file_name;
    tokio::fs::write(path, content).await
}

pub async fn remove_file(file_name: &str) -> tokio::io::Result<()> {
    let path = "./files".to_owned() + file_name;
    tokio::fs::remove_file(path).await
}
