use actix_web::{HttpRequest,get};
use crate::api::error::{ApiErr, ApiResult};

#[get("/")]
pub async fn index(_req: HttpRequest) -> ApiResult<String> {
    Ok("Hello from the index page!".to_string())
}