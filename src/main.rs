use std::net::TcpListener;
use ruchat_backend::configuration::get_config;
use ruchat_backend::run::run;
use sqlx::postgres::PgPool;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init calls set_logger
    // Printing all logs at info level if RUST_LOG environment variable not set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Make sure to panic if error while reading conf
    let configuration = get_config().expect("Failed to read config file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
