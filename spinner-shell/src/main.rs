//! SpinnerShell - SpinnerOS Desktop Environment

mod panel;
mod app_menu;
mod notifications;
mod theme;

use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application};
use libadwaita as adw;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::panel::Panel;

const APP_ID: &str = "org.spinneros.shell";

fn setup_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_shell=info".parse().unwrap()))
        .init();
}

fn main() -> glib::ExitCode {
    setup_logging();
    info!("Starting SpinnerShell v{}", env!("CARGO_PKG_VERSION"));
    
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();
    
    app.connect_startup(|_app| {
        info!("Application startup");
        setup_css();
    });
    
    app.connect_activate(|app| {
        info!("Application activated");
        build_ui(app);
    });
    
    let exit_code = app.run();
    info!("SpinnerShell exiting");
    exit_code
}

fn setup_css() {
    let provider = gtk4::CssProvider::new();
    let css_content = include_str!("theme/glass.css");
    provider.load_from_data(css_content);
    
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    info!("CSS theme loaded");
}

fn build_ui(app: &adw::Application) {
    let panel = Panel::new();
    let window = panel.create_window(app);
    window.present();
    info!("UI built and presented");
}
