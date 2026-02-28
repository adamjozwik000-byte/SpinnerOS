//! Theme management for SpinnerShell

use gtk4::prelude::*;
use gtk4::{gdk, CssProvider, Settings};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub variant: ThemeVariant,
    pub accent_color: String,
    pub background_color: String,
    pub foreground_color: String,
    pub blur_strength: f32,
    pub transparency: f32,
    pub border_radius: u32,
    pub shadow_intensity: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeVariant {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "SpinnerOS Glass".to_string(),
            variant: ThemeVariant::Dark,
            accent_color: "#88c0d0".to_string(),
            background_color: "#2e3440".to_string(),
            foreground_color: "#eceff4".to_string(),
            blur_strength: 0.6,
            transparency: 0.85,
            border_radius: 12,
            shadow_intensity: 0.3,
        }
    }
}

pub struct ThemeManager {
    current_theme: Theme,
    css_provider: CssProvider,
}

impl ThemeManager {
    pub fn new() -> Self {
        let css_provider = CssProvider::new();
        
        Self {
            current_theme: Theme::default(),
            css_provider,
        }
    }
    
    pub fn load_theme(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        self.current_theme = toml::from_str(&content)?;
        self.apply_theme();
        Ok(())
    }
    
    pub fn apply_theme(&self) {
        let css = self.generate_css();
        self.css_provider.load_from_string(&css);
        
        if let Some(display) = gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &self.css_provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        }
        
        info!("Theme applied: {}", self.current_theme.name);
    }
    
    pub fn set_variant(&mut self, variant: ThemeVariant) {
        self.current_theme.variant = variant;
        self.apply_theme();
        
        if let Some(settings) = Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(variant == ThemeVariant::Dark);
        }
    }
    
    pub fn set_accent_color(&mut self, color: &str) {
        self.current_theme.accent_color = color.to_string();
        self.apply_theme();
    }
    
    fn generate_css(&self) -> String {
        let theme = &self.current_theme;
        
        let (bg, fg, surface, shadow) = match theme.variant {
            ThemeVariant::Dark => (
                &theme.background_color,
                &theme.foreground_color,
                "rgba(46, 52, 64, 0.85)",
                "rgba(0, 0, 0, 0.3)",
            ),
            ThemeVariant::Light => (
                "#eceff4",
                "#2e3440",
                "rgba(236, 239, 244, 0.85)",
                "rgba(0, 0, 0, 0.1)",
            ),
        };
        
        format!(
            r#"
@define-color accent_color {accent};
@define-color bg_color {bg};
@define-color fg_color {fg};
@define-color surface_color {surface};
@define-color shadow_color {shadow};

* {{
    transition: all 200ms ease-out;
}}
"#,
            accent = theme.accent_color,
            bg = bg,
            fg = fg,
            surface = surface,
            shadow = shadow,
        )
    }
    
    pub fn current_theme(&self) -> &Theme {
        &self.current_theme
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_accent_colors() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Blue", "#88c0d0"),
        ("Green", "#a3be8c"),
        ("Purple", "#b48ead"),
        ("Orange", "#d08770"),
        ("Red", "#bf616a"),
        ("Yellow", "#ebcb8b"),
        ("Teal", "#8fbcbb"),
        ("Pink", "#e91e63"),
    ]
}
