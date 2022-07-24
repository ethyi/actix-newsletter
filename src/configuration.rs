use crate::domain::SubscriberEmail;
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgSslMode;
use sqlx::ConnectOptions;
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailClientSettings {
    pub base_url: String,     // api url
    pub sender_email: String, // email of sender
    pub authorization_token: Secret<String>,
    pub timeout_milliseconds: u64,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

// group detials regarding the app_address, set by spec.yaml by Digital ocean
#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    //  initialize config reader
    let mut settings = config::Config::default();
    // get base path
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    // get config file directory
    let configuration_directory = base_path.join("configuration");

    // add config values form a file named 'configuration'
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // detect running environment, parse string to enum
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // layer on higher level environment-specific values
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;
    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    // try to convert the configuration values it read into
    settings.try_into()
}
// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}
impl Environment {
    // environment in string format
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    // check if we need encrypted connection
    pub require_ssl: bool,
}

impl DatabaseSettings {
    // Renamed from `connection_string_without_db`
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    // Renamed from `connection_string`
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);

        options
    }
}
