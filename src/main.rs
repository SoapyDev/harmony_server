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

    let connection = PgPool::connect_lazy(&settings.database.connection_string().expose_secret())
        .expect("Failed to create Postgres pool.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application.port))?;
    run(listener, connection)?.await
}
