//! SpinnerOS Settings Application

use gtk4::prelude::*;
use gtk4::{self, gio, glib, Align, Box as GtkBox, Label, Orientation};
use libadwaita as adw;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const APP_ID: &str = "org.spinneros.settings";

fn main() -> glib::ExitCode {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_settings=info".parse().unwrap()))
        .init();
    
    info!("Starting SpinnerOS Settings");
    
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();
    
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Settings")
        .default_width(800)
        .default_height(600)
        .build();
    
    let main_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();
    
    let title = Label::builder()
        .label("SpinnerOS Settings")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    main_box.append(&title);
    
    let appearance_group = adw::PreferencesGroup::builder()
        .title("Appearance")
        .build();
    
    let dark_mode = adw::SwitchRow::builder()
        .title("Dark Mode")
        .subtitle("Use dark theme")
        .build();
    dark_mode.set_active(true);
    appearance_group.add(&dark_mode);
    
    main_box.append(&appearance_group);
    
    let about_group = adw::PreferencesGroup::builder()
        .title("About")
        .build();
    
    let version_row = adw::ActionRow::builder()
        .title("Version")
        .subtitle("SpinnerOS 0.1.0")
        .build();
    about_group.add(&version_row);
    
    main_box.append(&about_group);
    
    window.set_content(Some(&main_box));
    window.present();
}
