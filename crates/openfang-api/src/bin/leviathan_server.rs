//! Leviathan v3.4 Server — OpenFang-based Agent OS
//!
//! Standalone binary for Railway/Docker deployment.
//! Boots the OpenFang kernel with Leviathan agent roster,
//! starts the API server, and initializes the Discord bridge.

use openfang_kernel::OpenFangKernel;
use openfang_kernel::leviathan::LeviathanOS;
use std::path::PathBuf;

fn main() {
    // Initialize tracing for structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();

    tracing::info!("=== Leviathan v3.4.4 — OpenFang Agent OS ===");
    tracing::info!("Starting server...");

    // Determine config path
    let config_path = std::env::var("OPENFANG_CONFIG")
        .map(PathBuf::from)
        .ok()
        .or_else(|| {
            // Check common locations
            let paths = vec![
                PathBuf::from("config.toml"),
                PathBuf::from("/app/config.toml"),
                dirs::home_dir().map(|h| h.join(".openfang/config.toml")).unwrap_or_default(),
            ];
            paths.into_iter().find(|p| p.exists())
        });

    tracing::info!("Config path: {:?}", config_path);

    // Boot Leviathan OS (which boots the OpenFang kernel internally)
    let leviathan = match LeviathanOS::boot(config_path.as_deref()) {
        Ok(l) => {
            tracing::info!("Leviathan OS v{} booted successfully", l.version());
            l
        }
        Err(e) => {
            tracing::error!("FATAL: Failed to boot Leviathan: {}", e);
            std::process::exit(1);
        }
    };

    let kernel = leviathan.kernel();
    let listen_addr = kernel.config.api_listen.clone();

    tracing::info!("API server binding to {}", listen_addr);
    tracing::info!(
        "Default model: {}/{}",
        kernel.config.default_model.provider,
        kernel.config.default_model.model
    );
    tracing::info!("Agents loaded: {}", kernel.registry.count());

    // Start the async runtime and run the daemon
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        // Extract kernel from LeviathanOS for the daemon
        // We need to reconstruct since run_daemon takes ownership
        let kernel = OpenFangKernel::boot(config_path.as_deref())
            .expect("Second boot should not fail");

        if let Err(e) = openfang_api::server::run_daemon(kernel, &listen_addr, None).await {
            tracing::error!("Daemon error: {}", e);
            std::process::exit(1);
        }
    });
}
