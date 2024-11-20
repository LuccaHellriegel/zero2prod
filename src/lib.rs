use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    Ok(
        HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
            .listen(listener)?
            .run(),
    )
}
