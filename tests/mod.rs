mod allergy;
mod beneficiary;
mod connection;
mod message;
mod transaction;
mod user;
use harmony_server::configurations::get_configuration;
use harmony_server::startup::run;
use harmony_server::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

static TRACING: Lazy<()> = Lazy::new(|| {
    let susbcriber = get_subscriber("test".into(), "info".into());
    init_subscriber(susbcriber);
});

pub(crate) async fn spawn_app() -> String {
    Lazy::force(&TRACING);
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, connection).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
