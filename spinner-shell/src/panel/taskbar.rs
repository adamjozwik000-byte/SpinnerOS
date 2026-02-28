//! Taskbar - Window list in the panel

use gtk4::prelude::*;
use gtk4::{self, glib, Box as GtkBox, Button, Image, Label, Orientation, Popover};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct TaskbarItem {
    pub id: u32,
    pub title: String,
    pub app_id: String,
    pub icon_name: String,
    pub is_focused: bool,
    pub is_urgent: bool,
}

pub struct Taskbar {
    items: Rc<RefCell<Vec<TaskbarItem>>>,
    container: Rc<RefCell<Option<GtkBox>>>,
}

impl Taskbar {
    pub fn new() -> Self {
        Self {
            items: Rc::new(RefCell::new(Vec::new())),
            container: Rc::new(RefCell::new(None)),
        }
    }
    
    pub fn build_widget(&self) -> GtkBox {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .css_classes(["taskbar"])
            .build();
        
        *self.container.borrow_mut() = Some(container.clone());
        
        self.add_demo_items();
        self.refresh_ui();
        
        container
    }
    
    fn add_demo_items(&self) {
        let demo_items = vec![
            TaskbarItem {
                id: 1,
                title: "Files".to_string(),
                app_id: "org.gnome.Nautilus".to_string(),
                icon_name: "system-file-manager-symbolic".to_string(),
                is_focused: true,
                is_urgent: false,
            },
            TaskbarItem {
                id: 2,
                title: "Firefox".to_string(),
                app_id: "firefox".to_string(),
                icon_name: "firefox-symbolic".to_string(),
                is_focused: false,
                is_urgent: false,
            },
            TaskbarItem {
                id: 3,
                title: "Terminal".to_string(),
                app_id: "org.gnome.Terminal".to_string(),
                icon_name: "utilities-terminal-symbolic".to_string(),
                is_focused: false,
                is_urgent: false,
            },
        ];
        
        *self.items.borrow_mut() = demo_items;
    }
    
    pub fn add_item(&self, item: TaskbarItem) {
        self.items.borrow_mut().push(item);
        self.refresh_ui();
    }
    
    pub fn remove_item(&self, id: u32) {
        self.items.borrow_mut().retain(|item| item.id != id);
        self.refresh_ui();
    }
    
    pub fn set_focused(&self, id: u32) {
        for item in self.items.borrow_mut().iter_mut() {
            item.is_focused = item.id == id;
        }
        self.refresh_ui();
    }
    
    fn refresh_ui(&self) {
        let container = match self.container.borrow().clone() {
            Some(c) => c,
            None => return,
        };
        
        while let Some(child) = container.first_child() {
            container.remove(&child);
        }
        
        for item in self.items.borrow().iter() {
            let button = self.create_taskbar_button(item);
            container.append(&button);
        }
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
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .max_width_chars(15)
            .build();
        content.append(&label);
        
        let mut classes = vec!["taskbar-item"];
        if item.is_focused {
            classes.push("focused");
        }
        if item.is_urgent {
            classes.push("urgent");
        }
        
        let button = Button::builder()
            .child(&content)
            .tooltip_text(&item.title)
            .css_classes(classes)
            .build();
        
        let window_id = item.id;
        button.connect_clicked(move |_| {
            info!("Taskbar item clicked: window {}", window_id);
        });
        
        let popover = self.create_window_preview(item);
        
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(3);
        let popover_clone = popover.clone();
        gesture.connect_released(move |_, _, _, _| {
            popover_clone.popup();
        });
        button.add_controller(gesture);
        
        popover.set_parent(&button);
        
        button
    }
    
    fn create_window_preview(&self, item: &TaskbarItem) -> Popover {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .build();
        
        let title = Label::builder()
            .label(&item.title)
            .css_classes(["preview-title"])
            .build();
        content.append(&title);
        
        let preview_placeholder = GtkBox::builder()
            .width_request(200)
            .height_request(120)
            .css_classes(["window-preview"])
            .build();
        content.append(&preview_placeholder);
        
        let actions = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .halign(gtk4::Align::Center)
            .build();
        
        let close_btn = Button::builder()
            .icon_name("window-close-symbolic")
            .tooltip_text("Close")
            .css_classes(["preview-action"])
            .build();
        
        let window_id = item.id;
        close_btn.connect_clicked(move |_| {
            info!("Close window {} from preview", window_id);
        });
        
        actions.append(&close_btn);
        content.append(&actions);
        
        Popover::builder()
            .child(&content)
            .css_classes(["window-preview-popover"])
            .build()
    }
}

impl Default for Taskbar {
    fn default() -> Self {
        Self::new()
    }
}
