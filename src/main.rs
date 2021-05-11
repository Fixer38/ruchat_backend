use std::net::TcpListener;
use ruchat_backend::configuration::get_config;
use ruchat_backend::run::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Make sure to panic if error while reading conf
    let configuration = get_config().expect("Failed to read config file");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
