use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use crate::api::download::index;

fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(router)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}