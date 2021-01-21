use crate::api::download::download;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(download);
}

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(router))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
