use crate::libs::file;
use bytes::Bytes;
use hyper::StatusCode;

pub async fn read_file(path: &str) -> Result<Bytes, (StatusCode, String)> {
    file::read_file(path).await
}

pub async fn write_file(path: &str, content: Bytes) -> std::io::Result<()> {
    file::write_file(path, content).await
}

pub async fn remove_file(path: &str) -> std::io::Result<()> {
    file::remove_file(path).await
}

pub async fn remove_dir(path: &str) -> std::io::Result<()> {
    file::remove_dir(path).await
}
