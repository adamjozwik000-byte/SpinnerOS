//! Taskbar - Window list in the panel

use gtk4::prelude::*;
use gtk4::{self, Box as GtkBox, Button, Image, Label, Orientation};
use tracing::info;

#[derive(Debug, Clone)]
pub struct TaskbarItem {
    pub id: u32,
    pub title: String,
    pub icon_name: String,
    pub is_focused: bool,
}

pub struct Taskbar {
    items: Vec<TaskbarItem>,
}

impl Taskbar {
    pub fn new() -> Self {
        Self {
            items: vec![
                TaskbarItem {
                    id: 1,
                    title: "Files".to_string(),
                    icon_name: "system-file-manager-symbolic".to_string(),
                    is_focused: true,
                },
                TaskbarItem {
                    id: 2,
                    title: "Firefox".to_string(),
                    icon_name: "web-browser-symbolic".to_string(),
                    is_focused: false,
                },
                TaskbarItem {
                    id: 3,
                    title: "Terminal".to_string(),
                    icon_name: "utilities-terminal-symbolic".to_string(),
                    is_focused: false,
                },
            ],
        }
    }
    
    pub fn build_widget(&self) -> GtkBox {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        container.add_css_class("taskbar");
        
        for item in &self.items {
            let button = self.create_taskbar_button(item);
            container.append(&button);
        }
        
        container
    }
    
    fn create_taskbar_button(&self, item: &TaskbarItem) -> Button {
        let content = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();
        
        let icon = Image::builder()
            .icon_name(&item.icon_name)
            .pixel_size(20)
            .build();
        content.append(&icon);
        
        let label = Label::builder()
            .label(&item.title)
            .ellipsize(pango::EllipsizeMode::End)
            .max_width_chars(15)
            .build();
        content.append(&label);
        
        let button = Button::builder()
            .child(&content)
            .tooltip_text(&item.title)
            .build();
        button.add_css_class("taskbar-item");
        
        if item.is_focused {
            button.add_css_class("focused");
        }
        
        let window_id = item.id;
        button.connect_clicked(move |_| {
            info!("Taskbar item clicked: window {}", window_id);
        });
        
        button
    }
}

impl Default for Taskbar {
    fn default() -> Self {
        Self::new()
    }
}
