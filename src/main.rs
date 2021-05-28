use std::net::TcpListener;
use ruchat_backend::configuration::get_config;
use ruchat_backend::run::run;
use sqlx::postgres::PgPool;
use ruchat_backend::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("ruchat_backend".into(), "info".into());
    init_subscriber(subscriber);

    // Make sure to panic if error while reading conf
    let configuration = get_config().expect("Failed to read config file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
