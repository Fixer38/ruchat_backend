use std::net::TcpListener;
use crate::run::run;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use crate::configuration::{get_config, DatabaseSettings};
use std::collections::HashMap;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_config().expect("Failed to read config");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database)
        .await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to send request to health_check route");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn create_server_route() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let mut json_arg = HashMap::new();
    json_arg.insert("name", "Test Server By Request");

    let response = client
        .post(&format!("{}/server/create", &app.address))
        .json(&json_arg)
        .send()
        .await
        .expect("Failed to send request to create/server route");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}