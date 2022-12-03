use std::io::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub fn get_ext_name(file_name: &str) -> &str {
    let mut arr = file_name.rsplit(".");
    arr.next().unwrap_or("_")
}

pub async fn read_file(mut file: File) -> String {
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .await
        .unwrap_or_default();
    file_content
}
