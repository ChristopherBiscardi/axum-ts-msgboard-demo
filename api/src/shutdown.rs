use tokio::signal;
use tracing::{info, instrument};

#[instrument]
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(
            signal::unix::SignalKind::terminate(),
        )
        .expect("failed to install signal handler")
        .recv()
        .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("C-c");
            // Could write messages out to file then
            // read back in on startup
        },
        _ = terminate => {
            info!("terminate");
        },
    }

    println!("signal received, starting graceful shutdown");
}
