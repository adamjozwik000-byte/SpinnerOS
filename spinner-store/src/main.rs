//! SpinnerOS Software Store

use gtk4::prelude::*;
use gtk4::{self, gio, glib, Align, Box as GtkBox, Button, Label, Orientation, SearchEntry};
use libadwaita as adw;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const APP_ID: &str = "org.spinneros.store";

fn main() -> glib::ExitCode {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_store=info".parse().unwrap()))
        .init();
    
    info!("Starting SpinnerOS Software Store");
    
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
        .title("Software")
        .default_width(900)
        .default_height(650)
        .build();
    
    let main_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .build();
    
    // Header
    let header = adw::HeaderBar::builder().build();
    
    let search = SearchEntry::builder()
        .placeholder_text("Search applications...")
        .width_request(400)
        .build();
    header.set_title_widget(Some(&search));
    
    main_box.append(&header);
    
    // Content
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .vexpand(true)
        .build();
    
    let title = Label::builder()
        .label("Featured Apps")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-2");
    content.append(&title);
    
    let apps_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .build();
    
    let apps = vec![
        ("Firefox", "Web Browser", "web-browser-symbolic"),
        ("Files", "File Manager", "system-file-manager-symbolic"),
        ("Terminal", "Command Line", "utilities-terminal-symbolic"),
    ];
    
    for (name, desc, icon) in apps {
        let card = create_app_card(name, desc, icon);
        apps_box.append(&card);
    }
    
    content.append(&apps_box);
    main_box.append(&content);
    
    window.set_content(Some(&main_box));
    window.present();
}

fn create_app_card(name: &str, description: &str, icon_name: &str) -> GtkBox {
    let card = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .width_request(150)
        .build();
    card.add_css_class("card");
    
    let icon = gtk4::Image::builder()
        .icon_name(icon_name)
        .pixel_size(64)
        .margin_top(12)
        .build();
    card.append(&icon);
    
    let name_label = Label::builder()
        .label(name)
        .build();
    name_label.add_css_class("heading");
    card.append(&name_label);
    
    let desc_label = Label::builder()
        .label(description)
        .build();
    desc_label.add_css_class("dim-label");
    card.append(&desc_label);
    
    let install_btn = Button::builder()
        .label("Install")
        .margin_top(8)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    install_btn.add_css_class("suggested-action");
    
    let app_name = name.to_string();
    install_btn.connect_clicked(move |_| {
        info!("Install clicked: {}", app_name);
    });
    
    card.append(&install_btn);
    
    card
}
