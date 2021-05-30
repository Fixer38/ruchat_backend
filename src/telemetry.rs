use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;
use tracing_log::LogTracer;
use tracing::Subscriber;

// Using 'impl Subscriber' as return type instead of complicated Subscriber type
// Need for Send and Sync to be implemented to use 'init_subscriber' due to its async features
// Can take a sink as argument -> Place we write log to
pub fn get_subscriber(name: String, env_filter: String, sink: impl MakeWriter + Send + Sync + 'static) -> impl Subscriber + Send + Sync {
    // Printing all log spans at info level or above in case of failure
    // env_filter should be info if the logs have to be at info level
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        // Output formatted spans to the sink passed as an argument
        // Could be redirected to a remote location or dumped
        sink
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