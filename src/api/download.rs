use crate::api::error::{ApiErr, ApiResult};
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/api/v1/crates/{crate}/{version}/download")]
pub async fn download(path: web::Path<(String, String)>) -> ApiResult<HttpResponse> {
    let path = path.into_inner();
    info!("{:?}", path);
    Ok(HttpResponse::Ok().body(json!({
        "crate":path.0,
        "version":path.1,
    })))
}
