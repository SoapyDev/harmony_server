use harmony_server::configurations::get_configuration;
use harmony_server::startup::Application;
use harmony_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("harmony_server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Get configuration file
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
