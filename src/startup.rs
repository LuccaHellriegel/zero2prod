use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer,
};

use crate::routes::*;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    Ok(HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run())
}
