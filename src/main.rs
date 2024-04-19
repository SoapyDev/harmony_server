use harmony_server::configurations::get_configuration;
use harmony_server::startup::run;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use harmony_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("harmony_server".into(), "info".into());
    init_subscriber(subscriber);
    // Get configuration file
    let settings = get_configuration().expect("Failed to read configuration.");

    let connection = PgPool::connect(&settings.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))?;
    run(listener, connection)?.await
}
