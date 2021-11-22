use std::net::TcpListener;
use sqlx::{PgPool};
use email_project_rust::configuration::get_configuration;
use email_project_rust::startup::run;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration()
        .expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}