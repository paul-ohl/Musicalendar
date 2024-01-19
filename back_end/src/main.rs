#![warn(clippy::pedantic)]
// #![allow(unused)]

use axum::Router;
use std::net::SocketAddr;
use tracing::log::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod domain;
mod routes;
mod services;

// SETUP Constants
const SERVER_PORT: &str = "4321";
const SERVER_HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "musicalendar=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().merge(services::backend());

    let (port, host) = (SERVER_PORT, SERVER_HOST);

    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Cannot parse address and port");

    tracing::info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
