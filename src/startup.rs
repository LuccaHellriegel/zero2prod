use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer,
};
use sqlx::PgPool;

use crate::routes::*;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    Ok(HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run())
}
