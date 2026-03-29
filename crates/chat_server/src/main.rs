//! Chat server entry point.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use chat_server::{app, config, db, state};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,chat_server=debug")))
        .init();

    // Config
    let config_path = std::env::var("CHAT_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("config/config.dev.toml"));
    let cfg = config::ServerConfig::load(&config_path)?;
    let bind_addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    tracing::info!(host = %cfg.server.host, port = %cfg.server.port, "loaded config");

    // Database
    let pool = db::create_pool(&cfg.database.url, cfg.database.max_connections)
        .await
        .context("database pool")?;
    db::run_migrations(&pool).await?;
    tracing::info!("database ready");

    // Application state
    let state = Arc::new(state::AppState::new(pool, cfg));

    // Router
    let router = app::build_router(state.clone());

    // Bind & serve
    let listener = TcpListener::bind(&bind_addr).await.context("binding listener")?;
    tracing::info!(addr = %bind_addr, "listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(state.clone()))
        .await
        .context("server error")?;

    tracing::info!("server stopped");
    Ok(())
}

/// Wait for SIGINT or SIGTERM and initiate graceful shutdown.
async fn shutdown_signal(state: Arc<state::AppState>) {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("SIGINT received"),
        _ = terminate => tracing::info!("SIGTERM received"),
    }

    tracing::info!("initiating graceful shutdown");
    let _ = state.shutdown_tx.send(true);

    // Clear all sessions — dropping senders triggers outbound task shutdown.
    state.sessions.clear();

    // Brief grace period for in-flight messages.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
}
