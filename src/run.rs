use actix_web::{HttpServer, App, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::postgres::PgPool;
use crate::routes::health_check::health_check;
use crate::routes::server::create;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Middleware Logger added
            .wrap(TracingLogger)
            .route("/health_check", web::get().to(health_check))
            .route("/server/create", web::post().to(create))
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}