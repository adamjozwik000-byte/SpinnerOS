//! SpinnerWM - SpinnerOS Wayland Compositor
//!
//! A floating window manager built with Smithay for the SpinnerOS desktop environment.

mod compositor;
mod config;
mod input;
mod window;

use anyhow::Result;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::compositor::SpinnerCompositor;
use crate::config::Config;

fn setup_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_wm=info".parse().unwrap()))
        .init();
}

fn main() -> Result<()> {
    setup_logging();
    
    info!("Starting SpinnerWM v{}", env!("CARGO_PKG_VERSION"));
    
    let config = Config::load().unwrap_or_else(|e| {
        error!("Failed to load config: {}, using defaults", e);
        Config::default()
    });
    
    info!("Configuration loaded: {:?}", config);
    
    let mut compositor = SpinnerCompositor::new(config)?;
    
    info!("SpinnerWM compositor initialized");
    
    compositor.run()?;
    
    info!("SpinnerWM shutting down");
    Ok(())
}
