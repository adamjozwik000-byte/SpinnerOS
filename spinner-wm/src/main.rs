//! SpinnerWM - SpinnerOS Wayland Compositor

mod config;
mod input;
mod window;

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::config::Config;

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_wm=info".parse().unwrap()))
        .init();
    
    info!("Starting SpinnerWM v{}", env!("CARGO_PKG_VERSION"));
    
    let config = Config::load().unwrap_or_else(|e| {
        error!("Failed to load config: {}, using defaults", e);
        Config::default()
    });
    
    info!("Configuration loaded");
    info!("SpinnerWM initialized - this is a prototype");
    info!("Press Ctrl+C to exit");
    
    // Main event loop placeholder
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
