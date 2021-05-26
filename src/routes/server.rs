use actix_web::{web, HttpResponse, Error, error};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use tracing_futures::Instrument;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn create(form: web::Json<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    // Redirect Actix logs events to the Subscriber
    LogTracer::init().expect("Failed to set logger");

    // Printing all log spans at info level or above
    // If RUST_LOG environment variable is not set
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "ruchat_backend".into(),
        // Output formatted spans to stdout of terminal
        std::io::stdout
    );

    // 'With' method provided provided by 'SubscriberExt', extension trait of Subscriber
    // Given by 'tracing_subscriber'
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set Subscriber for log spans");

    let request_id = Uuid::new_v4();
    // Info span creates a span at the info level
    let request_span = tracing::info_span!(
        "Adding a new Server.",
        %request_id,
        name = %form.name
    );
    // Enter the request span returns an instance of Entered = a guard
    let _request_span = request_span.enter();
    let query_span = tracing::info_span!("Adding new Server to the database");
    sqlx::query!(
    r#"
    INSERT INTO servers (name, created_at)
    VALUES ($1, $2)
    "#,
    form.name,
    Utc::now()
    )
        .execute(pool.as_ref())
        .instrument(query_span)
        .await
        .map_err(|e| {
            // Using :? Debug format for deeper error messages
            tracing::error!("Request_id {} - Failed to execute query: {:?}", request_id, e);
            error::ErrorInternalServerError("Error From Server when executing Request")
        })?;
    tracing::info!("Request_id {} - New server details have been saved", request_id);
    Ok(HttpResponse::Ok().finish())
}