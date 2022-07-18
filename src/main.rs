use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::init_subscriber;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logging
    init_subscriber("newsletter".into(), "info".into(), std::io::stdout);

    // extract config and run server
    let configuration = get_configuration().expect("Failed to get config");
    let connection = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await?;
    Ok(())
}
