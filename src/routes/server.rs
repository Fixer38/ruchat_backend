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
    log::info!("Adding {} as a new server in the database.", form.name);
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
            log::error!("Failed to execute query: {:?}", e);
            error::ErrorInternalServerError("Error From Server when executing Request")
        })?;
    log::info!("New subscriber details have been saved");
    Ok(HttpResponse::Ok().finish())
}