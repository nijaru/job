pub mod server;
pub mod spawner;
pub mod state;

use crate::core::Paths;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

pub async fn run() -> Result<()> {
    let paths = Paths::new();
    paths.ensure_dirs()?;

    info!("Starting job daemon");
    info!("Socket: {}", paths.socket().display());
    info!("Database: {}", paths.database().display());

    let state = Arc::new(state::DaemonState::new(&paths)?);

    // Write PID file
    std::fs::write(paths.pid_file(), std::process::id().to_string())?;

    // Clean up stale socket
    if paths.socket().exists() {
        std::fs::remove_file(paths.socket())?;
    }

    // Run the server
    let result = server::run(paths.clone(), state.clone()).await;

    // Cleanup
    let _ = std::fs::remove_file(paths.pid_file());
    let _ = std::fs::remove_file(paths.socket());

    result
}
