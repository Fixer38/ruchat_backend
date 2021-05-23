use actix_web::{web, HttpResponse, Error, error};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn create(form: web::Json<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let request_id = Uuid::new_v4();
    // Info span creates a span at the info level
    let request_span = tracing::info_span!(
        "Adding a new Server.",
        %request_id,
        name = %form.name
    );
    // Enter the request span with a Guard
    let _request_span = request_span.enter();
    sqlx::query!(
    r#"
    INSERT INTO servers (name, created_at)
    VALUES ($1, $2)
    "#,
    form.name,
    Utc::now()
    )
        .execute(pool.as_ref())
        .await
        .map_err(|e| {
            // Using :? Debug format for deeper error messages
            tracing::error!("Request_id {} - Failed to execute query: {:?}", request_id, e);
            error::ErrorInternalServerError("Error From Server when executing Request")
        })?;
    tracing::info!("Request_id {} - New subscriber details have been saved", request_id);
    Ok(HttpResponse::Ok().finish())
}