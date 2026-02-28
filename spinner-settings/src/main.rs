//! SpinnerOS Settings Application
//!
//! System configuration and preferences

use gtk4::prelude::*;
use gtk4::{
    self, gio, glib, Align, Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, Stack, StackSidebar, Switch,
};
use libadwaita as adw;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const APP_ID: &str = "org.spinneros.settings";

fn setup_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("spinner_settings=info".parse().unwrap()))
        .init();
}

fn main() -> glib::ExitCode {
    setup_logging();
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
        .default_width(900)
        .default_height(650)
        .build();
    
    let main_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .build();
    
    let stack = Stack::builder()
        .transition_type(gtk4::StackTransitionType::SlideLeftRight)
        .build();
    
    stack.add_titled(&build_appearance_page(), Some("appearance"), "Appearance");
    stack.add_titled(&build_network_page(), Some("network"), "Network");
    stack.add_titled(&build_sound_page(), Some("sound"), "Sound");
    stack.add_titled(&build_displays_page(), Some("displays"), "Displays");
    stack.add_titled(&build_power_page(), Some("power"), "Power");
    stack.add_titled(&build_users_page(), Some("users"), "Users");
    stack.add_titled(&build_apps_page(), Some("apps"), "Applications");
    stack.add_titled(&build_about_page(), Some("about"), "About");
    
    let sidebar = StackSidebar::builder()
        .stack(&stack)
        .build();
    
    let sidebar_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .width_request(220)
        .build();
    
    let header = Label::builder()
        .label("Settings")
        .halign(Align::Start)
        .margin_top(20)
        .margin_bottom(16)
        .margin_start(20)
        .build();
    header.add_css_class("title-2");
    
    sidebar_box.append(&header);
    sidebar_box.append(&sidebar);
    
    main_box.append(&sidebar_box);
    main_box.append(&stack);
    
    window.set_content(Some(&main_box));
    window.present();
}

fn build_appearance_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Appearance")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let style_group = adw::PreferencesGroup::builder()
        .title("Style")
        .build();
    
    let dark_mode = adw::SwitchRow::builder()
        .title("Dark Mode")
        .subtitle("Use dark theme throughout the system")
        .build();
    dark_mode.set_active(true);
    style_group.add(&dark_mode);
    
    page.append(&style_group);
    
    let accent_group = adw::PreferencesGroup::builder()
        .title("Accent Color")
        .build();
    
    let colors_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(Align::Start)
        .margin_top(12)
        .build();
    
    let accent_colors = vec![
        ("#88c0d0", "Blue"),
        ("#a3be8c", "Green"),
        ("#b48ead", "Purple"),
        ("#d08770", "Orange"),
        ("#bf616a", "Red"),
        ("#ebcb8b", "Yellow"),
    ];
    
    for (color, name) in accent_colors {
        let btn = Button::builder()
            .width_request(36)
            .height_request(36)
            .tooltip_text(name)
            .build();
        
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&format!(
            "button {{ background: {}; border-radius: 50%; border: 2px solid transparent; }}
             button:hover {{ border-color: white; }}",
            color
        ));
        
        btn.style_context().add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
        colors_box.append(&btn);
    }
    
    let accent_row = adw::ActionRow::builder()
        .title("Accent Color")
        .subtitle("Choose your system accent color")
        .build();
    accent_row.add_suffix(&colors_box);
    accent_group.add(&accent_row);
    
    page.append(&accent_group);
    
    let wallpaper_group = adw::PreferencesGroup::builder()
        .title("Wallpaper")
        .build();
    
    let wallpaper_btn = Button::builder()
        .label("Change Wallpaper")
        .build();
    
    let wallpaper_row = adw::ActionRow::builder()
        .title("Desktop Background")
        .subtitle("Current: Default SpinnerOS")
        .build();
    wallpaper_row.add_suffix(&wallpaper_btn);
    wallpaper_group.add(&wallpaper_row);
    
    page.append(&wallpaper_group);
    
    page
}

fn build_network_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Network")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let wifi_group = adw::PreferencesGroup::builder()
        .title("Wi-Fi")
        .build();
    
    let wifi_switch = adw::SwitchRow::builder()
        .title("Wi-Fi")
        .subtitle("Connected to SpinnerOS-WiFi")
        .build();
    wifi_switch.set_active(true);
    wifi_group.add(&wifi_switch);
    
    page.append(&wifi_group);
    
    let networks_group = adw::PreferencesGroup::builder()
        .title("Available Networks")
        .build();
    
    let networks = vec![
        ("SpinnerOS-WiFi", "Connected", true),
        ("HomeNetwork", "Secured", false),
        ("Guest_5G", "Secured", false),
    ];
    
    for (name, status, connected) in networks {
        let row = adw::ActionRow::builder()
            .title(name)
            .subtitle(status)
            .build();
        
        if connected {
            let check = gtk4::Image::from_icon_name("emblem-ok-symbolic");
            row.add_suffix(&check);
        }
        
        networks_group.add(&row);
    }
    
    page.append(&networks_group);
    
    page
}

fn build_sound_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Sound")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let output_group = adw::PreferencesGroup::builder()
        .title("Output")
        .build();
    
    let volume_row = adw::ActionRow::builder()
        .title("Volume")
        .build();
    
    let volume_scale = gtk4::Scale::builder()
        .orientation(Orientation::Horizontal)
        .width_request(200)
        .build();
    volume_scale.set_range(0.0, 100.0);
    volume_scale.set_value(70.0);
    
    volume_row.add_suffix(&volume_scale);
    output_group.add(&volume_row);
    
    let output_device = adw::ComboRow::builder()
        .title("Output Device")
        .build();
    
    let model = gtk4::StringList::new(&["Built-in Speakers", "Headphones", "HDMI Audio"]);
    output_device.set_model(Some(&model));
    output_group.add(&output_device);
    
    page.append(&output_group);
    
    let input_group = adw::PreferencesGroup::builder()
        .title("Input")
        .build();
    
    let input_volume = adw::ActionRow::builder()
        .title("Input Volume")
        .build();
    
    let input_scale = gtk4::Scale::builder()
        .orientation(Orientation::Horizontal)
        .width_request(200)
        .build();
    input_scale.set_range(0.0, 100.0);
    input_scale.set_value(80.0);
    
    input_volume.add_suffix(&input_scale);
    input_group.add(&input_volume);
    
    page.append(&input_group);
    
    page
}

fn build_displays_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Displays")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let display_group = adw::PreferencesGroup::builder()
        .title("Built-in Display")
        .build();
    
    let resolution = adw::ComboRow::builder()
        .title("Resolution")
        .build();
    
    let res_model = gtk4::StringList::new(&["1920x1080", "1680x1050", "1440x900", "1366x768"]);
    resolution.set_model(Some(&res_model));
    display_group.add(&resolution);
    
    let refresh = adw::ComboRow::builder()
        .title("Refresh Rate")
        .build();
    
    let refresh_model = gtk4::StringList::new(&["60 Hz", "75 Hz", "120 Hz", "144 Hz"]);
    refresh.set_model(Some(&refresh_model));
    display_group.add(&refresh);
    
    let scale_row = adw::ComboRow::builder()
        .title("Scale")
        .build();
    
    let scale_model = gtk4::StringList::new(&["100%", "125%", "150%", "175%", "200%"]);
    scale_row.set_model(Some(&scale_model));
    display_group.add(&scale_row);
    
    let night_light = adw::SwitchRow::builder()
        .title("Night Light")
        .subtitle("Reduce blue light at night")
        .build();
    display_group.add(&night_light);
    
    page.append(&display_group);
    
    page
}

fn build_power_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Power")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let battery_group = adw::PreferencesGroup::builder()
        .title("Battery")
        .build();
    
    let battery_row = adw::ActionRow::builder()
        .title("Battery Level")
        .subtitle("85% - 3h 45m remaining")
        .build();
    battery_group.add(&battery_row);
    
    page.append(&battery_group);
    
    let power_group = adw::PreferencesGroup::builder()
        .title("Power Saving")
        .build();
    
    let power_saver = adw::SwitchRow::builder()
        .title("Power Saver Mode")
        .subtitle("Reduce performance to save battery")
        .build();
    power_group.add(&power_saver);
    
    let auto_suspend = adw::ComboRow::builder()
        .title("Automatic Suspend")
        .build();
    
    let suspend_model = gtk4::StringList::new(&["Never", "5 minutes", "10 minutes", "15 minutes", "30 minutes"]);
    auto_suspend.set_model(Some(&suspend_model));
    auto_suspend.set_selected(3);
    power_group.add(&auto_suspend);
    
    page.append(&power_group);
    
    page
}

fn build_users_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Users")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let user_group = adw::PreferencesGroup::builder()
        .title("Current User")
        .build();
    
    let user_row = adw::ActionRow::builder()
        .title("spinner")
        .subtitle("Administrator")
        .build();
    
    let avatar = adw::Avatar::builder()
        .size(48)
        .text("S")
        .build();
    user_row.add_prefix(&avatar);
    
    user_group.add(&user_row);
    
    let password_row = adw::ActionRow::builder()
        .title("Password")
        .subtitle("Last changed: Never")
        .activatable(true)
        .build();
    
    let arrow = gtk4::Image::from_icon_name("go-next-symbolic");
    password_row.add_suffix(&arrow);
    user_group.add(&password_row);
    
    page.append(&user_group);
    
    page
}

fn build_apps_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("Applications")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let default_group = adw::PreferencesGroup::builder()
        .title("Default Applications")
        .build();
    
    let defaults = vec![
        ("Web Browser", "Firefox"),
        ("Email", "Not set"),
        ("Music", "GNOME Music"),
        ("Video", "Videos"),
        ("Photos", "Image Viewer"),
        ("Files", "Files"),
    ];
    
    for (app_type, current) in defaults {
        let row = adw::ComboRow::builder()
            .title(app_type)
            .build();
        
        let model = gtk4::StringList::new(&[current]);
        row.set_model(Some(&model));
        default_group.add(&row);
    }
    
    page.append(&default_group);
    
    let startup_group = adw::PreferencesGroup::builder()
        .title("Startup Applications")
        .build();
    
    let add_startup = adw::ActionRow::builder()
        .title("Add Startup Application")
        .activatable(true)
        .build();
    
    let add_icon = gtk4::Image::from_icon_name("list-add-symbolic");
    add_startup.add_prefix(&add_icon);
    startup_group.add(&add_startup);
    
    page.append(&startup_group);
    
    page
}

fn build_about_page() -> GtkBox {
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(32)
        .margin_end(32)
        .build();
    
    let title = Label::builder()
        .label("About")
        .halign(Align::Start)
        .build();
    title.add_css_class("title-1");
    page.append(&title);
    
    let system_group = adw::PreferencesGroup::builder()
        .title("System Information")
        .build();
    
    let info = vec![
        ("Operating System", "SpinnerOS 0.1.0"),
        ("Kernel", "Linux 6.x"),
        ("Desktop", "SpinnerShell"),
        ("Window Manager", "SpinnerWM"),
        ("Architecture", "x86_64"),
    ];
    
    for (label, value) in info {
        let row = adw::ActionRow::builder()
            .title(label)
            .subtitle(value)
            .build();
        system_group.add(&row);
    }
    
    page.append(&system_group);
    
    let hardware_group = adw::PreferencesGroup::builder()
        .title("Hardware")
        .build();
    
    let hardware = vec![
        ("Processor", "Unknown"),
        ("Memory", "Unknown"),
        ("Graphics", "Unknown"),
        ("Disk", "Unknown"),
    ];
    
    for (label, value) in hardware {
        let row = adw::ActionRow::builder()
            .title(label)
            .subtitle(value)
            .build();
        hardware_group.add(&row);
    }
    
    page.append(&hardware_group);
    
    page
}
