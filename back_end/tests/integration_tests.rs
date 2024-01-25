use tokio::net::TcpListener;

use musicalendar::{
    domain::Calendar,
    startup::{app, settings},
};

/// # Panics
/// Will panic if the server cannot be started.
pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("Failed to bind random port.");
    let port = listener
        .local_addr()
        .expect("Couldn't retrive the addresss")
        .port();
    let address = format!("http://127.0.0.1:{port}");

    let mut settings = settings::build();
    settings.port = port;

    tokio::spawn(async move {
        axum::serve(listener, app(settings))
            .await
            .expect("Failed to start server");
    });
    address
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
    let response_text = response
        .text()
        .await
        .expect("Failed to read response body.");
    assert_eq!("OK", response_text);
}

#[tokio::test]
async fn get_calendar_returns_valid_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "playlist_link=https://open.spotify.com/playlist/2bLiTUO32rEAiBXCSiCLM7?si=e5777b59bd6b4e94";

    let response = client
        .post(format!("{}/api/calendar", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    let status = response.status().as_u16();
    let response_text = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to read response body.");
    let playlist: Calendar = serde_json::from_value(response_text).expect("Failed to parse json.");
    assert!(status == 200, "Response was not successful: {status}");
    assert_eq!(playlist.len(), 100);
}
