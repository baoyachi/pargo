use crate::api::download::download;
use crate::api::error::ApiResult;
use crate::api::new::new_crate;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(download).service(new_crate);
}

async fn not_found(req: HttpRequest) -> ApiResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}

pub async fn start_server() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(|| {
        App::new()
            .data(
                // change query extractor configuration
                web::QueryConfig::default().error_handler(|err, req| {
                    // <- create custom error response
                    error!("QueryConfig err:{:?}", err);

                    let resp = HttpResponse::BadRequest()
                        .set_header(http::header::CONTENT_TYPE, "application/json")
                        .body(json!({ "error": format!("{}", err) }));
                    InternalError::from_response(err, resp).into()
                }),
            )
            .configure(router)
            .wrap(Logger::default())
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
