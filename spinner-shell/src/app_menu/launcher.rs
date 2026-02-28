//! Application launcher window

use super::{create_app_button, hide_app_menu, AppCategory, AppEntry, SearchEngine};
use gtk4::prelude::*;
use gtk4::{
    self, gdk, glib, Align, Box as GtkBox, Button, Entry, FlowBox, Image, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, SelectionMode, Stack, StackSidebar, Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::info;

pub struct AppLauncher {
    apps: Vec<AppEntry>,
    search_engine: SearchEngine,
}

impl AppLauncher {
    pub fn new() -> Self {
        let apps = Self::load_applications();
        let search_engine = SearchEngine::new(apps.clone());
        
        Self {
            apps,
            search_engine,
        }
    }
    
    fn load_applications() -> Vec<AppEntry> {
        let mut apps = vec![
            AppEntry {
                name: "Files".to_string(),
                exec: "nautilus".to_string(),
                icon: "system-file-manager".to_string(),
                description: "Browse and manage files".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["file".to_string(), "manager".to_string(), "browse".to_string()],
            },
            AppEntry {
                name: "Firefox".to_string(),
                exec: "firefox".to_string(),
                icon: "firefox".to_string(),
                description: "Web Browser".to_string(),
                category: AppCategory::Internet,
                keywords: vec!["web".to_string(), "browser".to_string(), "internet".to_string()],
            },
            AppEntry {
                name: "Terminal".to_string(),
                exec: "gnome-terminal".to_string(),
                icon: "utilities-terminal".to_string(),
                description: "Terminal emulator".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["terminal".to_string(), "console".to_string(), "shell".to_string()],
            },
            AppEntry {
                name: "Text Editor".to_string(),
                exec: "gnome-text-editor".to_string(),
                icon: "text-editor".to_string(),
                description: "Edit text files".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["text".to_string(), "editor".to_string(), "notepad".to_string()],
            },
            AppEntry {
                name: "Settings".to_string(),
                exec: "spinner-settings".to_string(),
                icon: "preferences-system".to_string(),
                description: "System Settings".to_string(),
                category: AppCategory::System,
                keywords: vec!["settings".to_string(), "preferences".to_string(), "config".to_string()],
            },
            AppEntry {
                name: "Software".to_string(),
                exec: "spinner-store".to_string(),
                icon: "system-software-install".to_string(),
                description: "Install and manage applications".to_string(),
                category: AppCategory::System,
                keywords: vec!["software".to_string(), "install".to_string(), "apps".to_string(), "store".to_string()],
            },
            AppEntry {
                name: "Calculator".to_string(),
                exec: "gnome-calculator".to_string(),
                icon: "accessories-calculator".to_string(),
                description: "Perform calculations".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["calculator".to_string(), "math".to_string()],
            },
            AppEntry {
                name: "Image Viewer".to_string(),
                exec: "eog".to_string(),
                icon: "eog".to_string(),
                description: "View images".to_string(),
                category: AppCategory::Graphics,
                keywords: vec!["image".to_string(), "picture".to_string(), "photo".to_string(), "viewer".to_string()],
            },
            AppEntry {
                name: "Document Viewer".to_string(),
                exec: "evince".to_string(),
                icon: "evince".to_string(),
                description: "View PDF and other documents".to_string(),
                category: AppCategory::Office,
                keywords: vec!["pdf".to_string(), "document".to_string(), "viewer".to_string()],
            },
            AppEntry {
                name: "Videos".to_string(),
                exec: "totem".to_string(),
                icon: "totem".to_string(),
                description: "Play videos".to_string(),
                category: AppCategory::Multimedia,
                keywords: vec!["video".to_string(), "movie".to_string(), "player".to_string()],
            },
            AppEntry {
                name: "Music".to_string(),
                exec: "gnome-music".to_string(),
                icon: "gnome-music".to_string(),
                description: "Play music".to_string(),
                category: AppCategory::Multimedia,
                keywords: vec!["music".to_string(), "audio".to_string(), "player".to_string()],
            },
            AppEntry {
                name: "Screenshot".to_string(),
                exec: "gnome-screenshot".to_string(),
                icon: "applets-screenshooter".to_string(),
                description: "Take screenshots".to_string(),
                category: AppCategory::Utilities,
                keywords: vec!["screenshot".to_string(), "capture".to_string(), "screen".to_string()],
            },
        ];
        
        apps.sort_by(|a, b| a.name.cmp(&b.name));
        apps
    }
    
    pub fn create_window(&self) -> Window {
        let window = Window::builder()
            .title("Applications")
            .default_width(800)
            .default_height(600)
            .decorated(false)
            .modal(true)
            .css_classes(["app-menu-window"])
            .build();
        
        let main_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .css_classes(["app-menu-container"])
            .build();
        
        let header = self.build_header(&window);
        main_box.append(&header);
        
        let content = self.build_content();
        main_box.append(&content);
        
        window.set_child(Some(&main_box));
        
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                hide_app_menu();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        window.add_controller(key_controller);
        
        window
    }
    
    fn build_header(&self, window: &Window) -> GtkBox {
        let header = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .css_classes(["app-menu-header"])
            .build();
        
        let search_entry = Entry::builder()
            .placeholder_text("Search applications...")
            .hexpand(true)
            .css_classes(["app-search-entry"])
            .build();
        
        let search_icon = Image::builder()
            .icon_name("system-search-symbolic")
            .build();
        search_entry.set_primary_icon_paintable(Some(&search_icon.paintable().unwrap()));
        
        header.append(&search_entry);
        
        let close_button = Button::builder()
            .icon_name("window-close-symbolic")
            .css_classes(["app-menu-close"])
            .build();
        
        close_button.connect_clicked(|_| {
            hide_app_menu();
        });
        
        header.append(&close_button);
        
        header
    }
    
    fn build_content(&self) -> GtkBox {
        let content = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .vexpand(true)
            .css_classes(["app-menu-content"])
            .build();
        
        let sidebar = self.build_sidebar();
        content.append(&sidebar);
        
        let apps_container = self.build_apps_grid();
        content.append(&apps_container);
        
        content
    }
    
    fn build_sidebar(&self) -> GtkBox {
        let sidebar = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .width_request(200)
            .css_classes(["app-menu-sidebar"])
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .build();
        
        let all_btn = self.create_category_button("All Apps", "view-app-grid-symbolic", true);
        sidebar.append(&all_btn);
        
        for category in AppCategory::all() {
            let btn = self.create_category_button(category.label(), category.icon(), false);
            sidebar.append(&btn);
        }
        
        sidebar
    }
    
    fn create_category_button(&self, label: &str, icon_name: &str, active: bool) -> Button {
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
        
        let mut classes = vec!["category-button"];
        if active {
            classes.push("active");
        }
        
        let button = Button::builder()
            .child(&content)
            .css_classes(classes)
            .build();
        
        let label_str = label.to_string();
        button.connect_clicked(move |btn| {
            info!("Category selected: {}", label_str);
            
            if let Some(parent) = btn.parent() {
                let siblings = parent.observe_children();
                for i in 0..siblings.n_items() {
                    if let Some(sibling) = siblings.item(i) {
                        if let Ok(sibling_btn) = sibling.downcast::<Button>() {
                            sibling_btn.remove_css_class("active");
                        }
                    }
                }
            }
            btn.add_css_class("active");
        });
        
        button
    }
    
    fn build_apps_grid(&self) -> ScrolledWindow {
        let scrolled = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .css_classes(["app-grid-scroll"])
            .build();
        
        let flow_box = FlowBox::builder()
            .selection_mode(SelectionMode::None)
            .homogeneous(true)
            .min_children_per_line(4)
            .max_children_per_line(8)
            .row_spacing(16)
            .column_spacing(16)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .css_classes(["app-grid"])
            .build();
        
        for app in &self.apps {
            let child = create_app_button(app);
            flow_box.append(&child);
        }
        
        scrolled.set_child(Some(&flow_box));
        scrolled
    }
}

impl Default for AppLauncher {
    fn default() -> Self {
        Self::new()
    }
}
