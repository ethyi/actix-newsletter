// logging libraries
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// intialize logging
pub fn init_subscriber<Sink>(name: String, env_filter: String, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // initiate logger
    LogTracer::init().expect("Failed to set logger");

    // print all spans at the info-level if no enviroment variable RUST_LOG has been set
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(
        name, // Output the formatted spans to sink.
        sink,
    );

    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    // subscriber manages a span
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
}
