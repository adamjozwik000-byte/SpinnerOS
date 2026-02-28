//! Clock widget with calendar popup

use chrono::{Datelike, Local, Timelike};
use gtk4::prelude::*;
use gtk4::{self, glib, Box as GtkBox, Button, Calendar, Label, Orientation, Popover};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::debug;

pub struct Clock {
    time_label: Rc<RefCell<Option<Label>>>,
    date_label: Rc<RefCell<Option<Label>>>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            time_label: Rc::new(RefCell::new(None)),
            date_label: Rc::new(RefCell::new(None)),
        }
    }
    
    pub fn build_widget(&self) -> Button {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .valign(gtk4::Align::Center)
            .build();
        
        let time_label = Label::builder()
            .css_classes(["clock-time"])
            .build();
        content.append(&time_label);
        
        let date_label = Label::builder()
            .css_classes(["clock-date"])
            .build();
        content.append(&date_label);
        
        *self.time_label.borrow_mut() = Some(time_label.clone());
        *self.date_label.borrow_mut() = Some(date_label.clone());
        
        self.update_time();
        
        let button = Button::builder()
            .child(&content)
            .css_classes(["clock-button"])
            .build();
        
        let popover = self.create_calendar_popover();
        popover.set_parent(&button);
        
        button.connect_clicked(move |_| {
            popover.popup();
        });
        
        let time_label_clone = self.time_label.clone();
        let date_label_clone = self.date_label.clone();
        
        glib::timeout_add_seconds_local(1, move || {
            if let Some(ref label) = *time_label_clone.borrow() {
                let now = Local::now();
                label.set_label(&now.format("%H:%M").to_string());
            }
            if let Some(ref label) = *date_label_clone.borrow() {
                let now = Local::now();
                label.set_label(&now.format("%a, %b %d").to_string());
            }
            glib::ControlFlow::Continue
        });
        
        button
    }
    
    fn update_time(&self) {
        let now = Local::now();
        
        if let Some(ref label) = *self.time_label.borrow() {
            label.set_label(&now.format("%H:%M").to_string());
        }
        
        if let Some(ref label) = *self.date_label.borrow() {
            label.set_label(&now.format("%a, %b %d").to_string());
        }
    }
    
    fn create_calendar_popover(&self) -> Popover {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        
        let now = Local::now();
        
        let header = Label::builder()
            .label(&now.format("%B %Y").to_string())
            .css_classes(["calendar-header"])
            .build();
        content.append(&header);
        
        let big_time = Label::builder()
            .label(&now.format("%H:%M:%S").to_string())
            .css_classes(["calendar-big-time"])
            .build();
        content.append(&big_time);
        
        glib::timeout_add_seconds_local(1, move || {
            let now = Local::now();
            big_time.set_label(&now.format("%H:%M:%S").to_string());
            glib::ControlFlow::Continue
        });
        
        let calendar = Calendar::builder()
            .css_classes(["calendar-widget"])
            .build();
        
        calendar.connect_day_selected(|cal| {
            let date = cal.date();
            debug!("Selected date: {:?}", date);
        });
        
        content.append(&calendar);
        
        let events_section = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .css_classes(["events-section"])
            .build();
        
        let events_header = Label::builder()
            .label("Today's Events")
            .halign(gtk4::Align::Start)
            .css_classes(["events-header"])
            .build();
        events_section.append(&events_header);
        
        let no_events = Label::builder()
            .label("No events scheduled")
            .halign(gtk4::Align::Start)
            .css_classes(["no-events"])
            .build();
        events_section.append(&no_events);
        
        content.append(&events_section);
        
        Popover::builder()
            .child(&content)
            .css_classes(["calendar-popover"])
            .build()
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
