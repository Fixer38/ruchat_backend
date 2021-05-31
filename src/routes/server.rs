use actix_web::{web, HttpResponse, Error, error};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use tracing_futures::Instrument;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

#[tracing::instrument(
    name = "Creating a new Server",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        name = %form.name
    )
)]
pub async fn create(form: web::Json<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
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
            tracing::error!("Failed to execute query: {:?}", e);
            error::ErrorInternalServerError("Error From Server when executing Request")
        })?;
    Ok(HttpResponse::Ok().finish())
}