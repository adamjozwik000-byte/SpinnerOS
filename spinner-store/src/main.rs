//! SpinnerOS Software Store
//!
//! GUI for managing APT and Flatpak packages

use gtk4::prelude::*;
use gtk4::{
    self, gio, glib, Align, Box as GtkBox, Button, Entry, FlowBox, FlowBoxChild, Image, Label,
    Orientation, ScrolledWindow, SearchEntry, Stack, StackSwitcher,
};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const APP_ID: &str = "org.spinneros.store";

fn setup_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_store=info".parse().unwrap()))
        .init();
}

fn main() -> glib::ExitCode {
    setup_logging();
    info!("Starting SpinnerOS Software Store");
    
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();
    
    app.connect_activate(build_ui);
    
    app.run()
}

#[derive(Debug, Clone)]
struct AppInfo {
    name: String,
    summary: String,
    icon: String,
    category: String,
    source: PackageSource,
    installed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageSource {
    Apt,
    Flatpak,
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Software")
        .default_width(1000)
        .default_height(700)
        .build();
    
    let main_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .build();
    
    let header = build_header();
    main_box.append(&header);
    
    let content = build_content();
    main_box.append(&content);
    
    window.set_content(Some(&main_box));
    window.present();
}

fn build_header() -> adw::HeaderBar {
    let header = adw::HeaderBar::builder()
        .build();
    
    let search = SearchEntry::builder()
        .placeholder_text("Search applications...")
        .width_request(400)
        .build();
    
    search.connect_search_changed(|entry| {
        let query = entry.text();
        info!("Searching for: {}", query);
    });
    
    header.set_title_widget(Some(&search));
    
    let menu_btn = Button::builder()
        .icon_name("open-menu-symbolic")
        .build();
    header.pack_end(&menu_btn);
    
    header
}

fn build_content() -> GtkBox {
    let content = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .build();
    
    let sidebar = build_sidebar();
    content.append(&sidebar);
    
    let main_area = build_main_area();
    content.append(&main_area);
    
    content
}

fn build_sidebar() -> GtkBox {
    let sidebar = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .width_request(220)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .build();
    
    let categories = vec![
        ("Explore", "compass-symbolic", true),
        ("Installed", "view-grid-symbolic", false),
        ("Updates", "software-update-available-symbolic", false),
        ("", "", false),
        ("Productivity", "x-office-document-symbolic", false),
        ("Development", "applications-engineering-symbolic", false),
        ("Graphics", "applications-graphics-symbolic", false),
        ("Internet", "web-browser-symbolic", false),
        ("Multimedia", "applications-multimedia-symbolic", false),
        ("Games", "applications-games-symbolic", false),
        ("Utilities", "applications-utilities-symbolic", false),
    ];
    
    for (name, icon, active) in categories {
        if name.is_empty() {
            let separator = gtk4::Separator::new(Orientation::Horizontal);
            separator.set_margin_top(8);
            separator.set_margin_bottom(8);
            sidebar.append(&separator);
            continue;
        }
        
        let btn = create_sidebar_button(name, icon, active);
        sidebar.append(&btn);
    }
    
    sidebar
}

fn create_sidebar_button(label: &str, icon_name: &str, active: bool) -> Button {
    let content = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    
    let icon = Image::builder()
        .icon_name(icon_name)
        .pixel_size(20)
        .build();
    content.append(&icon);
    
    let label_widget = Label::builder()
        .label(label)
        .halign(Align::Start)
        .hexpand(true)
        .build();
    content.append(&label_widget);
    
    let btn = Button::builder()
        .child(&content)
        .build();
    
    if active {
        btn.add_css_class("suggested-action");
    }
    
    let label_str = label.to_string();
    btn.connect_clicked(move |_| {
        info!("Category selected: {}", label_str);
    });
    
    btn
}

fn build_main_area() -> GtkBox {
    let main_area = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(24)
        .build();
    
    let featured = build_featured_section();
    main_area.append(&featured);
    
    let editor_picks = build_app_section("Editor's Picks", get_sample_apps());
    main_area.append(&editor_picks);
    
    let recently_updated = build_app_section("Recently Updated", get_sample_apps());
    main_area.append(&recently_updated);
    
    main_area
}

fn build_featured_section() -> GtkBox {
    let section = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();
    
    let header = Label::builder()
        .label("Featured")
        .halign(Align::Start)
        .build();
    header.add_css_class("title-2");
    section.append(&header);
    
    let carousel = adw::Carousel::builder()
        .hexpand(true)
        .height_request(200)
        .build();
    
    let featured_apps = vec![
        ("Firefox", "Browse the web freely", "firefox"),
        ("Visual Studio Code", "Code editing. Redefined.", "code"),
        ("GIMP", "GNU Image Manipulation Program", "gimp"),
    ];
    
    for (name, desc, icon) in featured_apps {
        let card = build_featured_card(name, desc, icon);
        carousel.append(&card);
    }
    
    let dots = adw::CarouselIndicatorDots::builder()
        .carousel(&carousel)
        .build();
    
    section.append(&carousel);
    section.append(&dots);
    
    section
}

fn build_featured_card(name: &str, description: &str, icon: &str) -> GtkBox {
    let card = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(24)
        .hexpand(true)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    card.add_css_class("card");
    
    let icon_widget = Image::builder()
        .icon_name(icon)
        .pixel_size(128)
        .build();
    card.append(&icon_widget);
    
    let info = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .valign(Align::Center)
        .hexpand(true)
        .build();
    
    let name_label = Label::builder()
        .label(name)
        .halign(Align::Start)
        .build();
    name_label.add_css_class("title-1");
    info.append(&name_label);
    
    let desc_label = Label::builder()
        .label(description)
        .halign(Align::Start)
        .build();
    info.append(&desc_label);
    
    let install_btn = Button::builder()
        .label("Install")
        .halign(Align::Start)
        .margin_top(12)
        .build();
    install_btn.add_css_class("suggested-action");
    info.append(&install_btn);
    
    card.append(&info);
    
    card
}

fn build_app_section(title: &str, apps: Vec<AppInfo>) -> GtkBox {
    let section = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();
    
    let header_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .build();
    
    let header = Label::builder()
        .label(title)
        .halign(Align::Start)
        .hexpand(true)
        .build();
    header.add_css_class("title-2");
    header_box.append(&header);
    
    let see_all = Button::builder()
        .label("See All")
        .build();
    see_all.add_css_class("flat");
    header_box.append(&see_all);
    
    section.append(&header_box);
    
    let flow = FlowBox::builder()
        .homogeneous(true)
        .min_children_per_line(3)
        .max_children_per_line(6)
        .row_spacing(12)
        .column_spacing(12)
        .selection_mode(gtk4::SelectionMode::None)
        .build();
    
    for app in apps {
        let card = build_app_card(&app);
        flow.append(&card);
    }
    
    section.append(&flow);
    
    section
}

fn build_app_card(app: &AppInfo) -> FlowBoxChild {
    let card = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .width_request(160)
        .build();
    
    card.add_css_class("card");
    
    let icon = Image::builder()
        .icon_name(&app.icon)
        .pixel_size(64)
        .margin_top(12)
        .build();
    card.append(&icon);
    
    let name = Label::builder()
        .label(&app.name)
        .max_width_chars(15)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();
    name.add_css_class("heading");
    card.append(&name);
    
    let summary = Label::builder()
        .label(&app.summary)
        .max_width_chars(20)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();
    summary.add_css_class("dim-label");
    card.append(&summary);
    
    let source_label = match app.source {
        PackageSource::Apt => "deb",
        PackageSource::Flatpak => "flatpak",
    };
    
    let badge = Label::builder()
        .label(source_label)
        .margin_bottom(12)
        .build();
    badge.add_css_class("dim-label");
    card.append(&badge);
    
    let button = Button::builder()
        .child(&card)
        .build();
    button.add_css_class("flat");
    
    let app_name = app.name.clone();
    button.connect_clicked(move |_| {
        info!("Opening app details: {}", app_name);
    });
    
    FlowBoxChild::builder()
        .child(&button)
        .build()
}

fn get_sample_apps() -> Vec<AppInfo> {
    vec![
        AppInfo {
            name: "Firefox".to_string(),
            summary: "Web Browser".to_string(),
            icon: "firefox".to_string(),
            category: "Internet".to_string(),
            source: PackageSource::Apt,
            installed: true,
        },
        AppInfo {
            name: "GIMP".to_string(),
            summary: "Image Editor".to_string(),
            icon: "gimp".to_string(),
            category: "Graphics".to_string(),
            source: PackageSource::Flatpak,
            installed: false,
        },
        AppInfo {
            name: "VLC".to_string(),
            summary: "Media Player".to_string(),
            icon: "vlc".to_string(),
            category: "Multimedia".to_string(),
            source: PackageSource::Apt,
            installed: false,
        },
        AppInfo {
            name: "LibreOffice".to_string(),
            summary: "Office Suite".to_string(),
            icon: "libreoffice-startcenter".to_string(),
            category: "Office".to_string(),
            source: PackageSource::Apt,
            installed: false,
        },
        AppInfo {
            name: "VS Code".to_string(),
            summary: "Code Editor".to_string(),
            icon: "visual-studio-code".to_string(),
            category: "Development".to_string(),
            source: PackageSource::Flatpak,
            installed: false,
        },
        AppInfo {
            name: "Spotify".to_string(),
            summary: "Music Streaming".to_string(),
            icon: "spotify".to_string(),
            category: "Multimedia".to_string(),
            source: PackageSource::Flatpak,
            installed: false,
        },
    ]
}
