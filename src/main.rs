use harmony_server::configurations::get_configuration;
use harmony_server::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

use harmony_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("harmony_server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Get configuration file
    let settings = get_configuration().expect("Failed to read configuration.");

    let connection = PgPool::connect_lazy_with(settings.database.with_db());

    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))?;
    run(listener, connection)?.await
}
