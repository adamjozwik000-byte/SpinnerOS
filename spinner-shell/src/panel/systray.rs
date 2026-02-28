//! System Tray - Network, sound, battery indicators

use gtk4::prelude::*;
use gtk4::{self, glib, Box as GtkBox, Button, Image, Label, Orientation, Popover, Scale};
use tracing::info;

pub struct SystemTray {
    network_status: NetworkStatus,
    audio_level: u32,
    battery_level: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
pub enum NetworkStatus {
    Disconnected,
    Wired,
    Wifi(u32),
    Cellular(u32),
}

impl SystemTray {
    pub fn new() -> Self {
        Self {
            network_status: NetworkStatus::Wifi(75),
            audio_level: 70,
            battery_level: Some(85),
        }
    }
    
    pub fn build_widget(&self) -> GtkBox {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .css_classes(["systray"])
            .build();
        
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
        let icon_name = match self.network_status {
            NetworkStatus::Disconnected => "network-offline-symbolic",
            NetworkStatus::Wired => "network-wired-symbolic",
            NetworkStatus::Wifi(strength) => {
                if strength > 75 {
                    "network-wireless-signal-excellent-symbolic"
                } else if strength > 50 {
                    "network-wireless-signal-good-symbolic"
                } else if strength > 25 {
                    "network-wireless-signal-ok-symbolic"
                } else {
                    "network-wireless-signal-weak-symbolic"
                }
            }
            NetworkStatus::Cellular(strength) => {
                if strength > 75 {
                    "network-cellular-signal-excellent-symbolic"
                } else if strength > 50 {
                    "network-cellular-signal-good-symbolic"
                } else {
                    "network-cellular-signal-weak-symbolic"
                }
            }
        };
        
        let button = Button::builder()
            .icon_name(icon_name)
            .tooltip_text("Network")
            .css_classes(["systray-button", "network-indicator"])
            .build();
        
        let popover = self.create_network_popover();
        popover.set_parent(&button);
        
        button.connect_clicked(move |_| {
            popover.popup();
        });
        
        button
    }
    
    fn create_network_popover(&self) -> Popover {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .width_request(280)
            .build();
        
        let header = Label::builder()
            .label("Network")
            .css_classes(["popover-header"])
            .halign(gtk4::Align::Start)
            .build();
        content.append(&header);
        
        let wifi_row = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        
        let wifi_icon = Image::builder()
            .icon_name("network-wireless-signal-excellent-symbolic")
            .pixel_size(24)
            .build();
        wifi_row.append(&wifi_icon);
        
        let wifi_info = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .build();
        
        let wifi_name = Label::builder()
            .label("SpinnerOS-WiFi")
            .halign(gtk4::Align::Start)
            .css_classes(["network-name"])
            .build();
        wifi_info.append(&wifi_name);
        
        let wifi_status = Label::builder()
            .label("Connected")
            .halign(gtk4::Align::Start)
            .css_classes(["network-status"])
            .build();
        wifi_info.append(&wifi_status);
        
        wifi_row.append(&wifi_info);
        content.append(&wifi_row);
        
        let settings_btn = Button::builder()
            .label("Network Settings")
            .css_classes(["popover-button"])
            .build();
        
        settings_btn.connect_clicked(|_| {
            info!("Open network settings");
        });
        
        content.append(&settings_btn);
        
        Popover::builder()
            .child(&content)
            .css_classes(["systray-popover"])
            .build()
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
            .css_classes(["systray-button", "audio-indicator"])
            .build();
        
        let popover = self.create_audio_popover();
        popover.set_parent(&button);
        
        button.connect_clicked(move |_| {
            popover.popup();
        });
        
        button
    }
    
    fn create_audio_popover(&self) -> Popover {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .width_request(280)
            .build();
        
        let header = Label::builder()
            .label("Sound")
            .css_classes(["popover-header"])
            .halign(gtk4::Align::Start)
            .build();
        content.append(&header);
        
        let volume_row = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        
        let volume_icon = Image::builder()
            .icon_name("audio-volume-high-symbolic")
            .pixel_size(24)
            .build();
        volume_row.append(&volume_icon);
        
        let volume_scale = Scale::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(true)
            .build();
        volume_scale.set_range(0.0, 100.0);
        volume_scale.set_value(self.audio_level as f64);
        volume_scale.set_draw_value(false);
        
        volume_scale.connect_value_changed(|scale| {
            let value = scale.value() as u32;
            info!("Volume changed to {}%", value);
        });
        
        volume_row.append(&volume_scale);
        content.append(&volume_row);
        
        let output_label = Label::builder()
            .label("Output Device")
            .halign(gtk4::Align::Start)
            .css_classes(["section-label"])
            .margin_top(8)
            .build();
        content.append(&output_label);
        
        let output_btn = Button::builder()
            .css_classes(["device-button"])
            .build();
        
        let output_content = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        
        let speaker_icon = Image::builder()
            .icon_name("audio-speakers-symbolic")
            .pixel_size(20)
            .build();
        output_content.append(&speaker_icon);
        
        let speaker_label = Label::builder()
            .label("Built-in Speakers")
            .hexpand(true)
            .halign(gtk4::Align::Start)
            .build();
        output_content.append(&speaker_label);
        
        output_btn.set_child(Some(&output_content));
        content.append(&output_btn);
        
        Popover::builder()
            .child(&content)
            .css_classes(["systray-popover"])
            .build()
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
        } else if level > 10 {
            "battery-level-20-symbolic"
        } else {
            "battery-level-10-symbolic"
        };
        
        let button = Button::builder()
            .icon_name(icon_name)
            .tooltip_text(&format!("Battery: {}%", level))
            .css_classes(["systray-button", "battery-indicator"])
            .build();
        
        let popover = self.create_battery_popover(level);
        popover.set_parent(&button);
        
        button.connect_clicked(move |_| {
            popover.popup();
        });
        
        button
    }
    
    fn create_battery_popover(&self, level: u32) -> Popover {
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .width_request(250)
            .build();
        
        let header = Label::builder()
            .label("Battery")
            .css_classes(["popover-header"])
            .halign(gtk4::Align::Start)
            .build();
        content.append(&header);
        
        let level_label = Label::builder()
            .label(&format!("{}%", level))
            .css_classes(["battery-level-big"])
            .build();
        content.append(&level_label);
        
        let status_label = Label::builder()
            .label("3h 45m remaining")
            .css_classes(["battery-status"])
            .build();
        content.append(&status_label);
        
        let settings_btn = Button::builder()
            .label("Power Settings")
            .css_classes(["popover-button"])
            .build();
        
        settings_btn.connect_clicked(|_| {
            info!("Open power settings");
        });
        
        content.append(&settings_btn);
        
        Popover::builder()
            .child(&content)
            .css_classes(["systray-popover"])
            .build()
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new()
    }
}
