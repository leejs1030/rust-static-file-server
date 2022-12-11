use crate::StatusCode;
use bytes::Bytes;

pub fn get_ext_name(file_name: &str) -> &str {
    let mut arr = file_name.rsplit(".");
    arr.next().unwrap_or("_")
}

pub async fn read_file(file_name: &str) -> Result<Bytes, (StatusCode, String)> {
    let path = "./files".to_owned() + file_name;
    let file = tokio::fs::read(path).await;
    match file {
        Ok(contents) => Ok(Bytes::from(contents)),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                Err((StatusCode::NOT_FOUND, String::from("File does not exist!")))
            }
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        },
    }
}

pub async fn write_file(file_name: &str, content: Bytes) -> tokio::io::Result<()> {
    let path = "./files".to_owned() + file_name;
    let last = path.rfind("/").unwrap();

    let true_path = &path[..last];
    tokio::fs::create_dir_all(true_path).await.unwrap();
    tokio::fs::write(path, content).await
}

pub async fn remove_file(file_name: &str) -> tokio::io::Result<()> {
    let path = "./files".to_owned() + file_name;
    tokio::fs::remove_file(path).await
}

pub async fn remove_dir(file_name: &str) -> tokio::io::Result<()> {
    let path = "./files".to_owned() + file_name;
    tokio::fs::remove_dir_all(path).await
}
