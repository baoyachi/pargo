use crate::api::download::download;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::api::error::ApiResult;
use actix_web::http::StatusCode;
use crate::api::new::new;
use actix_web::middleware::Logger;

fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(download)
    .service(new);
}

async fn not_found(req:HttpRequest) -> ApiResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}

pub async fn start_server() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(||
        App::new()
            .configure(router)
            .wrap(Logger::default())
            .default_service(web::route().to(not_found))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
