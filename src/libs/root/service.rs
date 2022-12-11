use crate::libs::file;
use crate::StatusCode;
use bytes::Bytes;
use std::collections::HashMap;

pub async fn read_file_by_path(path: &str) -> Result<Bytes, (StatusCode, String)> {
    file::read_file(path).await
}

pub async fn write_file(path: &str, content: Bytes) -> std::io::Result<()> {
    file::write_file(path, content).await
}

pub fn check_query_is_directory(query_map: &HashMap<&str, &str>) -> bool {
    let tmp = *query_map.get("type").unwrap_or(&"file");
    match tmp {
        "directory" => true,
        _ => false,
    }
}

pub async fn delete_by_path_and_query(
    path: &str,
    query_map: &HashMap<&str, &str>,
) -> std::io::Result<()> {
    let is_directory = check_query_is_directory(&query_map);
    match is_directory {
        false => file::remove_file(path).await,
        true => file::remove_dir(path).await,
    }
}
