use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn create(form: web::Json<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
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
            eprintln!("Failed to execute create server query: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}