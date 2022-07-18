use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection); // wrap over ARC
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default()) // middle ware added with wrap, emits log record at every request
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
