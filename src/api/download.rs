use crate::api::error::{ApiErr, ApiResult};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web::body::ResponseBody;
use std::fs;

#[get("/api/v1/crates/{crate}/{version}/download")]
pub async fn download(path: web::Path<(String, String)>) -> ApiResult<HttpResponse> {
    let path = path.into_inner();
    info!("{:?}", path);
    let body = fs::read("rust_demo.crates").unwrap();
    Ok(HttpResponse::Ok().body(body))
}
