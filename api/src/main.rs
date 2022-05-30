use api::{make_router, shutdown::shutdown_signal};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = make_router();
    // needs to be [0,0,0,0] in a container,
    // or [::]:3000 on fly.io to handle ipv6 and ipv4
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
