use newsletter::configuration::get_configuration;
use newsletter::startup::Application;
use newsletter::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logging
    init_subscriber("newsletter".into(), "info".into(), std::io::stdout);

    // extract config and run server
    let configuration = get_configuration().expect("Failed to get config");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
