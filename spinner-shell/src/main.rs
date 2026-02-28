//! SpinnerShell - SpinnerOS Desktop Environment
//!
//! A modern, GTK4-based desktop shell with glass neomorphism styling.

mod panel;
mod app_menu;
mod notifications;
mod theme;

use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application};
use libadwaita as adw;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::panel::Panel;
use crate::theme::ThemeManager;

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
    
    gio::resources_register_include!("spinner-shell.gresource")
        .unwrap_or_else(|e| {
            info!("No gresource bundle found ({}), using filesystem resources", e);
        });
    
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();
    
    app.connect_startup(|app| {
        info!("Application startup");
        setup_css();
        setup_app(app);
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
    provider.load_from_string(css_content);
    
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    info!("CSS theme loaded");
}

fn setup_app(app: &adw::Application) {
    let quit_action = gio::SimpleAction::new("quit", None);
    let app_clone = app.clone();
    quit_action.connect_activate(move |_, _| {
        app_clone.quit();
    });
    app.add_action(&quit_action);
    
    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(|_, _| {
        show_about_dialog();
    });
    app.add_action(&about_action);
}

fn build_ui(app: &adw::Application) {
    let panel = Panel::new();
    
    let window = panel.create_window(app);
    window.present();
    
    info!("UI built and presented");
}

fn show_about_dialog() {
    let dialog = adw::AboutDialog::builder()
        .application_name("SpinnerShell")
        .version(env!("CARGO_PKG_VERSION"))
        .developer_name("SpinnerOS Team")
        .license_type(gtk4::License::Gpl30)
        .website("https://spinneros.org")
        .issue_url("https://github.com/spinneros/spinneros/issues")
        .application_icon("spinneros-logo")
        .build();
    
    dialog.present(None::<&gtk4::Window>);
}
