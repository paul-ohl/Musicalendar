use musicalendar::startup::{app, settings};

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "musicalendar=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = settings::build();
    let addr: SocketAddr = format!("{}:{}", &settings.host, &settings.port)
        .parse()
        .expect("Cannot parse address and port");
    tracing::info!("listening on http://{}", addr);
    let app = app(settings);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
