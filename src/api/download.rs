use actix_web::{HttpRequest, get, web};
use crate::api::error::{ApiErr, ApiResult};

#[get("/api/v1/crates/{crate}/{version}/download")]
pub async fn download(path: web::Path<(String, String)>) -> ApiResult<String> {
    info!("{:?}", path);
    Ok("Hello from the index page!".to_string())
}