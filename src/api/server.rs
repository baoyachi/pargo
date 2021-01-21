use crate::api::download::download;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::api::error::ApiResult;
use actix_web::http::StatusCode;
use crate::api::new::new;

fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(download);
    // .service(new);
}

async fn not_found(req:HttpRequest) -> ApiResult<HttpResponse> {
    info!("{:?}",req);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .configure(router)
            .default_service(web::route().to(not_found))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
