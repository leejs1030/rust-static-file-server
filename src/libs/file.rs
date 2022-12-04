use std::io::prelude::*;
use bytes::Bytes;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub fn get_ext_name(file_name: &str) -> &str {
    let mut arr = file_name.rsplit(".");
    arr.next().unwrap_or("_")
}

pub async fn read_file(file_name: &str) -> Result<Vec<u8>, String> {
    let file = tokio::fs::read(file_name).await;
    match file{
        Ok(contents) => Ok(contents),
        Err(e) => Err(e.to_string()),
    }
}
