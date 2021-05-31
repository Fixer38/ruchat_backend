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
    insert_server(&form, &pool);
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Saving new Server details in the database",
    skip(form, pool)
)]
pub async fn insert_server(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
    r#"
    INSERT INTO servers (name, created_at)
    VALUES ($1, $2)
    "#,
    form.name,
    Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            // Using :? Debug format for deeper error messages
            tracing::error!("Failed to execute query: {:?}", e);
            e
            //error::ErrorInternalServerError("Error From Server when executing Request")
        })?;
    Ok(())
}