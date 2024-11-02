use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = api::router::create_router()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!(
        "Listening backend server on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
