//! Panel module - Top bar with taskbar, system tray, and clock

mod taskbar;
mod systray;
mod clock;

pub use taskbar::Taskbar;
pub use systray::SystemTray;
pub use clock::Clock;

use gtk4::prelude::*;
use gtk4::{self, gdk, glib, Align, Box as GtkBox, Button, Orientation};
use libadwaita as adw;
use tracing::info;

const PANEL_HEIGHT: i32 = 48;

pub struct Panel {
    taskbar: Taskbar,
    systray: SystemTray,
    clock: Clock,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            taskbar: Taskbar::new(),
            systray: SystemTray::new(),
            clock: Clock::new(),
        }
    }
    
    pub fn create_window(&self, app: &adw::Application) -> gtk4::ApplicationWindow {
        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .decorated(false)
            .resizable(false)
            .default_height(PANEL_HEIGHT)
            .css_classes(["panel-window"])
            .build();
        
        setup_panel_layer(&window);
        
        let main_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .hexpand(true)
            .css_classes(["panel"])
            .build();
        
        let left_section = self.build_left_section();
        let center_section = self.build_center_section();
        let right_section = self.build_right_section();
        
        main_box.append(&left_section);
        main_box.append(&center_section);
        main_box.append(&right_section);
        
        window.set_child(Some(&main_box));
        
        info!("Panel window created");
        window
    }
    
    fn build_left_section(&self) -> GtkBox {
        let section = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_start(12)
            .halign(Align::Start)
            .css_classes(["panel-section", "panel-left"])
            .build();
        
        let menu_button = Button::builder()
            .icon_name("view-app-grid-symbolic")
            .tooltip_text("Applications")
            .css_classes(["panel-button", "app-menu-button"])
            .build();
        
        menu_button.connect_clicked(|_| {
            info!("App menu clicked");
            crate::app_menu::show_app_menu();
        });
        
        section.append(&menu_button);
        
        let taskbar_widget = self.taskbar.build_widget();
        section.append(&taskbar_widget);
        
        section
    }
    
    fn build_center_section(&self) -> GtkBox {
        let section = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .halign(Align::Center)
            .hexpand(true)
            .css_classes(["panel-section", "panel-center"])
            .build();
        
        let workspaces = self.build_workspace_indicators();
        section.append(&workspaces);
        
        section
    }
    
    fn build_right_section(&self) -> GtkBox {
        let section = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_end(12)
            .halign(Align::End)
            .css_classes(["panel-section", "panel-right"])
            .build();
        
        let systray_widget = self.systray.build_widget();
        section.append(&systray_widget);
        
        let clock_widget = self.clock.build_widget();
        section.append(&clock_widget);
        
        let power_button = Button::builder()
            .icon_name("system-shutdown-symbolic")
            .tooltip_text("Power")
            .css_classes(["panel-button", "power-button"])
            .build();
        
        power_button.connect_clicked(|_| {
            info!("Power button clicked");
        });
        
        section.append(&power_button);
        
        section
    }
    
    fn build_workspace_indicators(&self) -> GtkBox {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .css_classes(["workspace-indicators"])
            .build();
        
        for i in 1..=5 {
            let indicator = Button::builder()
                .label(&i.to_string())
                .css_classes(["workspace-indicator"])
                .build();
            
            if i == 1 {
                indicator.add_css_class("active");
            }
            
            let workspace_num = i;
            indicator.connect_clicked(move |btn| {
                info!("Workspace {} clicked", workspace_num);
                
                if let Some(parent) = btn.parent() {
                    let siblings = parent.observe_children();
                    for j in 0..siblings.n_items() {
                        if let Some(sibling) = siblings.item(j) {
                            if let Ok(sibling_btn) = sibling.downcast::<Button>() {
                                sibling_btn.remove_css_class("active");
                            }
                        }
                    }
                }
                btn.add_css_class("active");
            });
            
            container.append(&indicator);
        }
        
        container
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_panel_layer(window: &gtk4::ApplicationWindow) {
    window.set_default_size(
        gdk::Display::default()
            .map(|d| {
                d.monitors()
                    .item(0)
                    .and_then(|m| m.downcast::<gdk::Monitor>().ok())
                    .map(|m| m.geometry().width())
                    .unwrap_or(1920)
            })
            .unwrap_or(1920),
        PANEL_HEIGHT,
    );
}
