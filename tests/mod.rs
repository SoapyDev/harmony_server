mod allergy;
mod beneficiary;
mod connection;
mod message;
mod transaction;
mod user;
use harmony_server::configurations::get_configuration;
use harmony_server::startup::run;
use harmony_server::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

pub(crate) async fn spawn_app() -> String {
    let subscriber = get_subscriber("test".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, connection).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
