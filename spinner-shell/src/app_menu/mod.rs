//! Application menu with launcher and search

mod launcher;
mod search;

pub use launcher::AppLauncher;
pub use search::SearchEngine;

use gtk4::prelude::*;
use gtk4::{
    self, gdk, glib, Align, Box as GtkBox, Button, Entry, FlowBox, FlowBoxChild, Image, Label,
    Orientation, ScrolledWindow, Window, WindowType,
};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::info;

static APP_MENU_WINDOW: std::sync::OnceLock<Rc<RefCell<Option<Window>>>> = std::sync::OnceLock::new();

pub fn show_app_menu() {
    let window_cell = APP_MENU_WINDOW.get_or_init(|| Rc::new(RefCell::new(None)));
    
    if let Some(ref window) = *window_cell.borrow() {
        if window.is_visible() {
            window.set_visible(false);
            return;
        } else {
            window.set_visible(true);
            window.present();
            return;
        }
    }
    
    let launcher = AppLauncher::new();
    let window = launcher.create_window();
    
    *window_cell.borrow_mut() = Some(window.clone());
    window.present();
}

pub fn hide_app_menu() {
    let window_cell = APP_MENU_WINDOW.get_or_init(|| Rc::new(RefCell::new(None)));
    
    if let Some(ref window) = *window_cell.borrow() {
        window.set_visible(false);
    }
}

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub description: String,
    pub category: AppCategory,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCategory {
    Favorites,
    Internet,
    Development,
    Office,
    Graphics,
    Multimedia,
    Games,
    System,
    Utilities,
    Other,
}

impl AppCategory {
    pub fn label(&self) -> &'static str {
        match self {
            AppCategory::Favorites => "Favorites",
            AppCategory::Internet => "Internet",
            AppCategory::Development => "Development",
            AppCategory::Office => "Office",
            AppCategory::Graphics => "Graphics",
            AppCategory::Multimedia => "Multimedia",
            AppCategory::Games => "Games",
            AppCategory::System => "System",
            AppCategory::Utilities => "Utilities",
            AppCategory::Other => "Other",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            AppCategory::Favorites => "starred-symbolic",
            AppCategory::Internet => "web-browser-symbolic",
            AppCategory::Development => "applications-engineering-symbolic",
            AppCategory::Office => "x-office-document-symbolic",
            AppCategory::Graphics => "applications-graphics-symbolic",
            AppCategory::Multimedia => "applications-multimedia-symbolic",
            AppCategory::Games => "applications-games-symbolic",
            AppCategory::System => "preferences-system-symbolic",
            AppCategory::Utilities => "applications-utilities-symbolic",
            AppCategory::Other => "application-x-executable-symbolic",
        }
    }
    
    pub fn all() -> Vec<AppCategory> {
        vec![
            AppCategory::Favorites,
            AppCategory::Internet,
            AppCategory::Development,
            AppCategory::Office,
            AppCategory::Graphics,
            AppCategory::Multimedia,
            AppCategory::Games,
            AppCategory::System,
            AppCategory::Utilities,
            AppCategory::Other,
        ]
    }
}

pub fn create_app_button(app: &AppEntry) -> FlowBoxChild {
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(100)
        .height_request(100)
        .build();
    
    let icon = Image::builder()
        .icon_name(&app.icon)
        .pixel_size(48)
        .css_classes(["app-icon"])
        .build();
    content.append(&icon);
    
    let label = Label::builder()
        .label(&app.name)
        .max_width_chars(12)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .css_classes(["app-label"])
        .build();
    content.append(&label);
    
    let button = Button::builder()
        .child(&content)
        .tooltip_text(&app.description)
        .css_classes(["app-button"])
        .build();
    
    let exec = app.exec.clone();
    button.connect_clicked(move |_| {
        info!("Launching: {}", exec);
        
        let parts: Vec<&str> = exec.split_whitespace().collect();
        if !parts.is_empty() {
            if let Err(e) = std::process::Command::new(parts[0])
                .args(&parts[1..])
                .spawn()
            {
                info!("Failed to launch {}: {}", exec, e);
            }
        }
        
        hide_app_menu();
    });
    
    let child = FlowBoxChild::builder()
        .child(&button)
        .css_classes(["app-flow-child"])
        .build();
    
    child
}
