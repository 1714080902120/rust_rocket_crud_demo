use rocket::{http::Status, tokio::fs};
use std::path::Path;

pub async fn write_into_md(
    user_id: &str,
    article_id: &str,
    content_buf: &[u8],
) -> Result<(), Status> {
    let file_path = format_md_path(user_id, article_id);
    match fs::write(Path::new(&file_path), content_buf).await {
        Err(err) => {
            dbg!(err);
            return Err(Status::InternalServerError);
        }
        Ok(_) => return Ok(()),
    }
}

pub async fn read_md_into_str(user_id: &str, article_id: &str) -> Result<String, Status> {
    let file_path = format_md_path(user_id, article_id);
    match fs::read_to_string(Path::new(&file_path)).await {
        Err(err) => {
            dbg!(err);
            return Err(Status::InternalServerError);
        }
        Ok(content) => return Ok(content),
    }
}

pub fn format_md_path(user_id: &str, article_id: &str) -> String {
    format!("md/{user_id}/{article_id}.md")
}
