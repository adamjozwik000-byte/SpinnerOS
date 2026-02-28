//! Panel module - Top bar with taskbar, system tray, and clock

mod taskbar;
mod systray;
mod clock;

pub use taskbar::Taskbar;
pub use systray::SystemTray;
pub use clock::Clock;

use gtk4::prelude::*;
use gtk4::{self, Align, Box as GtkBox, Button, Orientation};
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
            .build();
        
        window.add_css_class("panel-window");
        
        let main_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .hexpand(true)
            .build();
        main_box.add_css_class("panel");
        
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
            .build();
        section.add_css_class("panel-section");
        section.add_css_class("panel-left");
        
        let menu_button = Button::builder()
            .icon_name("view-app-grid-symbolic")
            .tooltip_text("Applications")
            .build();
        menu_button.add_css_class("panel-button");
        menu_button.add_css_class("app-menu-button");
        
        menu_button.connect_clicked(|_| {
            info!("App menu clicked");
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
            .build();
        section.add_css_class("panel-section");
        section.add_css_class("panel-center");
        
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
            .build();
        section.add_css_class("panel-section");
        section.add_css_class("panel-right");
        
        let systray_widget = self.systray.build_widget();
        section.append(&systray_widget);
        
        let clock_widget = self.clock.build_widget();
        section.append(&clock_widget);
        
        let power_button = Button::builder()
            .icon_name("system-shutdown-symbolic")
            .tooltip_text("Power")
            .build();
        power_button.add_css_class("panel-button");
        power_button.add_css_class("power-button");
        
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
            .build();
        container.add_css_class("workspace-indicators");
        
        for i in 1..=5 {
            let indicator = Button::builder()
                .label(&i.to_string())
                .build();
            indicator.add_css_class("workspace-indicator");
            
            if i == 1 {
                indicator.add_css_class("active");
            }
            
            let workspace_num = i;
            indicator.connect_clicked(move |btn| {
                info!("Workspace {} clicked", workspace_num);
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
