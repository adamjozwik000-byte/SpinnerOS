//! System Tray - Network, sound, battery indicators

use gtk4::prelude::*;
use gtk4::{self, Box as GtkBox, Button, Orientation};
use tracing::info;

pub struct SystemTray {
    audio_level: u32,
    battery_level: Option<u32>,
}

impl SystemTray {
    pub fn new() -> Self {
        Self {
            audio_level: 70,
            battery_level: Some(85),
        }
    }
    
    pub fn build_widget(&self) -> GtkBox {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        container.add_css_class("systray");
        
        let network_btn = self.build_network_indicator();
        container.append(&network_btn);
        
        let audio_btn = self.build_audio_indicator();
        container.append(&audio_btn);
        
        if let Some(level) = self.battery_level {
            let battery_btn = self.build_battery_indicator(level);
            container.append(&battery_btn);
        }
        
        container
    }
    
    fn build_network_indicator(&self) -> Button {
        let button = Button::builder()
            .icon_name("network-wireless-signal-excellent-symbolic")
            .tooltip_text("Network")
            .build();
        button.add_css_class("systray-button");
        button.add_css_class("network-indicator");
        
        button.connect_clicked(|_| {
            info!("Network indicator clicked");
        });
        
        button
    }
    
    fn build_audio_indicator(&self) -> Button {
        let icon_name = if self.audio_level == 0 {
            "audio-volume-muted-symbolic"
        } else if self.audio_level < 33 {
            "audio-volume-low-symbolic"
        } else if self.audio_level < 66 {
            "audio-volume-medium-symbolic"
        } else {
            "audio-volume-high-symbolic"
        };
        
        let button = Button::builder()
            .icon_name(icon_name)
            .tooltip_text(&format!("Volume: {}%", self.audio_level))
            .build();
        button.add_css_class("systray-button");
        button.add_css_class("audio-indicator");
        
        button.connect_clicked(|_| {
            info!("Audio indicator clicked");
        });
        
        button
    }
    
    fn build_battery_indicator(&self, level: u32) -> Button {
        let icon_name = if level > 80 {
            "battery-level-100-symbolic"
        } else if level > 60 {
            "battery-level-80-symbolic"
        } else if level > 40 {
            "battery-level-60-symbolic"
        } else if level > 20 {
            "battery-level-40-symbolic"
        } else {
            "battery-level-20-symbolic"
        };
        
        let button = Button::builder()
            .icon_name(icon_name)
            .tooltip_text(&format!("Battery: {}%", level))
            .build();
        button.add_css_class("systray-button");
        button.add_css_class("battery-indicator");
        
        button.connect_clicked(|_| {
            info!("Battery indicator clicked");
        });
        
        button
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new()
    }
}
