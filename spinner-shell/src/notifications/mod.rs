//! Notification center for SpinnerShell

use gtk4::prelude::*;
use gtk4::{
    self, glib, Align, Box as GtkBox, Button, Image, Label, ListBox, ListBoxRow, Orientation,
    Revealer, RevealerTransitionType, ScrolledWindow,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub icon: String,
    pub urgency: Urgency,
    pub timestamp: SystemTime,
    pub actions: Vec<NotificationAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
}

pub struct NotificationCenter {
    notifications: Rc<RefCell<Vec<Notification>>>,
    container: Rc<RefCell<Option<GtkBox>>>,
    next_id: Rc<RefCell<u32>>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: Rc::new(RefCell::new(Vec::new())),
            container: Rc::new(RefCell::new(None)),
            next_id: Rc::new(RefCell::new(1)),
        }
    }
    
    pub fn build_widget(&self) -> GtkBox {
        let main_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .css_classes(["notification-center"])
            .build();
        
        let header = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .css_classes(["notification-header"])
            .build();
        
        let title = Label::builder()
            .label("Notifications")
            .halign(Align::Start)
            .hexpand(true)
            .css_classes(["notification-title"])
            .build();
        header.append(&title);
        
        let clear_btn = Button::builder()
            .label("Clear All")
            .css_classes(["notification-clear"])
            .build();
        
        let notifications = self.notifications.clone();
        let container = self.container.clone();
        clear_btn.connect_clicked(move |_| {
            notifications.borrow_mut().clear();
            if let Some(ref cont) = *container.borrow() {
                while let Some(child) = cont.first_child() {
                    cont.remove(&child);
                }
            }
            info!("All notifications cleared");
        });
        
        header.append(&clear_btn);
        main_box.append(&header);
        
        let scrolled = ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .css_classes(["notification-scroll"])
            .build();
        
        let list_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .css_classes(["notification-list"])
            .build();
        
        *self.container.borrow_mut() = Some(list_box.clone());
        
        if self.notifications.borrow().is_empty() {
            let empty_label = Label::builder()
                .label("No notifications")
                .css_classes(["notification-empty"])
                .vexpand(true)
                .valign(Align::Center)
                .build();
            list_box.append(&empty_label);
        }
        
        scrolled.set_child(Some(&list_box));
        main_box.append(&scrolled);
        
        main_box
    }
    
    pub fn add_notification(&self, notification: Notification) {
        let id = notification.id;
        self.notifications.borrow_mut().push(notification.clone());
        
        if let Some(ref container) = *self.container.borrow() {
            if container.first_child().map_or(false, |c| {
                c.css_classes().contains(&glib::GString::from("notification-empty"))
            }) {
                if let Some(child) = container.first_child() {
                    container.remove(&child);
                }
            }
            
            let widget = self.create_notification_widget(&notification);
            container.prepend(&widget);
        }
        
        info!("Notification added: {} - {}", notification.app_name, notification.summary);
    }
    
    pub fn remove_notification(&self, id: u32) {
        self.notifications.borrow_mut().retain(|n| n.id != id);
        info!("Notification {} removed", id);
    }
    
    pub fn notify(
        &self,
        app_name: &str,
        summary: &str,
        body: &str,
        icon: &str,
        urgency: Urgency,
    ) -> u32 {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let notification = Notification {
            id,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            icon: icon.to_string(),
            urgency,
            timestamp: SystemTime::now(),
            actions: Vec::new(),
        };
        
        self.add_notification(notification);
        self.show_popup(id);
        
        id
    }
    
    fn show_popup(&self, id: u32) {
        info!("Showing popup for notification {}", id);
    }
    
    fn create_notification_widget(&self, notification: &Notification) -> GtkBox {
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .css_classes(["notification-item"])
            .build();
        
        match notification.urgency {
            Urgency::Critical => container.add_css_class("critical"),
            Urgency::Low => container.add_css_class("low"),
            _ => {}
        }
        
        let header = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();
        
        let icon = Image::builder()
            .icon_name(&notification.icon)
            .pixel_size(24)
            .css_classes(["notification-icon"])
            .build();
        header.append(&icon);
        
        let app_label = Label::builder()
            .label(&notification.app_name)
            .halign(Align::Start)
            .hexpand(true)
            .css_classes(["notification-app"])
            .build();
        header.append(&app_label);
        
        let close_btn = Button::builder()
            .icon_name("window-close-symbolic")
            .css_classes(["notification-close"])
            .build();
        
        let notifications = self.notifications.clone();
        let container_ref = self.container.clone();
        let id = notification.id;
        let container_widget = container.clone();
        
        close_btn.connect_clicked(move |_| {
            notifications.borrow_mut().retain(|n| n.id != id);
            
            if let Some(ref cont) = *container_ref.borrow() {
                cont.remove(&container_widget);
            }
            
            info!("Notification {} dismissed", id);
        });
        
        header.append(&close_btn);
        container.append(&header);
        
        let summary = Label::builder()
            .label(&notification.summary)
            .halign(Align::Start)
            .css_classes(["notification-summary"])
            .build();
        container.append(&summary);
        
        if !notification.body.is_empty() {
            let body = Label::builder()
                .label(&notification.body)
                .halign(Align::Start)
                .wrap(true)
                .max_width_chars(40)
                .css_classes(["notification-body"])
                .build();
            container.append(&body);
        }
        
        if !notification.actions.is_empty() {
            let actions_box = GtkBox::builder()
                .orientation(Orientation::Horizontal)
                .spacing(8)
                .margin_top(8)
                .halign(Align::End)
                .build();
            
            for action in &notification.actions {
                let btn = Button::builder()
                    .label(&action.label)
                    .css_classes(["notification-action"])
                    .build();
                
                let action_id = action.id.clone();
                btn.connect_clicked(move |_| {
                    info!("Notification action triggered: {}", action_id);
                });
                
                actions_box.append(&btn);
            }
            
            container.append(&actions_box);
        }
        
        container
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}
