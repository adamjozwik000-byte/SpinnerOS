//! Application menu module

mod launcher;
mod search;

pub use launcher::AppLauncher;
pub use search::SearchEngine;

use tracing::info;

pub fn show_app_menu() {
    info!("Showing app menu");
}

pub fn hide_app_menu() {
    info!("Hiding app menu");
}

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub description: String,
    pub keywords: Vec<String>,
}
