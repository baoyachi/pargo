// 2021-01-22 01:43:37:654200000 [INFO] <actix_server::builder:263>:Starting 4 workers
// 2021-01-22 01:43:37:655028000 [INFO] <actix_server::builder:277>:Starting "actix-web-service-127.0.0.1:8080" service on 127.0.0.1:8080
// 2021-01-22 01:49:13:270352000 [INFO] <pargo::api::server:11>:
// HttpRequest HTTP/1.1 PUT:/api/v1/crates/new
// headers:
// "accept": "application/json"
// "expect": "100-continue"
// "content-length": "73183"
// "authorization": "xxxadmin"
// "host": "localhost:8080"
// "user-agent": "cargo 1.49.0 (d00d64df9 2020-12-05)"
//
//

use actix_web::{HttpRequest, HttpResponse, put, web, HttpMessage};
use crate::api::error::ApiResult;

#[put("/api/v1/crates/new")]
pub async fn new(mut req: HttpRequest) -> ApiResult<HttpResponse> {
    info!("req:{:?}", req);
    Ok(HttpResponse::Ok().finish())
}
