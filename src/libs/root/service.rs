use super::repository;
use crate::StatusCode;
use bytes::Bytes;
use std::collections::HashMap;

pub async fn read_file(path: &str) -> Result<Bytes, (StatusCode, String)> {
    repository::read_file(path).await
}

pub async fn write_file(path: &str, content: Bytes) -> std::io::Result<()> {
    repository::write_file(path, content).await
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
        false => repository::remove_file(path).await,
        true => repository::remove_dir(path).await,
    }
}
