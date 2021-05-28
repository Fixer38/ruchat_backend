use std::net::TcpListener;
use ruchat_backend::configuration::get_config;
use ruchat_backend::run::run;
use sqlx::postgres::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing::Subscriber;

// Using 'impl Subscriber' as return type instead of complicated Subscriber type
// Need for Send and Sync to be implemented to use 'init_subscriber' due to its async features
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    // Printing all log spans at info level or above in case of failure
    // env_filter should be info if the logs have to be at info level
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        // Output formatted spans to the terminal stdout
        // Could be redirected to a remote location or dumped
        std::io::stdout
    );
    // 'with' method provided by 'SubscriberExt', Extension trait of Subscriber
    // Given by 'tracing_subscriber'
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Register subscriber as the global and default way to process span data
// Should only be called once
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Initialize the redirection of Actix logs into Subscriber
    LogTracer::init().expect("Failed to set logger");
    // Set the global tracing to the created subscriber
    set_global_default(subscriber).expect("Failed to get subscriber");
}

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
