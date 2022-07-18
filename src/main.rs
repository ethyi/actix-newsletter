use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::init_subscriber;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logging
    init_subscriber("newsletter".into(), "info".into(), std::io::stdout);

    // extract config and run server
    let configuration = get_configuration().expect("Failed to get config");
    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await?;
    Ok(())
}
