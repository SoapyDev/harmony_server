use harmony_server::configurations::get_configuration;
use harmony_server::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Get configuration file
    let settings = get_configuration().expect("Failed to read configuration.");

    let connection = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))?;
    run(listener, connection)?.await
}
