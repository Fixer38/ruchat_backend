use actix_web::{web, App, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;

#[derive(Deserialize)]
pub struct Info {
    name: String,
}

pub async fn create(info: web::Json<Info>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse>{
    sqlx::query!(
        r#"
        INSERT INTO servers (name, created_at)
        VALUES ($1, $2)
        "#,
        info.name,
        Utc::now()
    )
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to execute create server query: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}