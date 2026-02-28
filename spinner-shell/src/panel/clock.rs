//! Clock widget with calendar popup

use chrono::Local;
use gtk4::prelude::*;
use gtk4::{self, glib, Box as GtkBox, Button, Label, Orientation};

pub struct Clock {}

impl Clock {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn build_widget(&self) -> Button {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .valign(gtk4::Align::Center)
            .build();
        
        let time_label = Label::builder().build();
        time_label.add_css_class("clock-time");
        content.append(&time_label);
        
        let date_label = Label::builder().build();
        date_label.add_css_class("clock-date");
        content.append(&date_label);
        
        // Initial update
        let now = Local::now();
        time_label.set_label(&now.format("%H:%M").to_string());
        date_label.set_label(&now.format("%a, %b %d").to_string());
        
        let button = Button::builder()
            .child(&content)
            .build();
        button.add_css_class("clock-button");
        
        // Update every second
        let time_label_clone = time_label.clone();
        let date_label_clone = date_label.clone();
        
        glib::timeout_add_seconds_local(1, move || {
            let now = Local::now();
            time_label_clone.set_label(&now.format("%H:%M").to_string());
            date_label_clone.set_label(&now.format("%a, %b %d").to_string());
            glib::ControlFlow::Continue
        });
        
        button
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
