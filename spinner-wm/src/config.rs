//! Configuration management for SpinnerWM

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub appearance: AppearanceConfig,
    pub keybindings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub focus_follows_mouse: bool,
    pub cursor_theme: String,
    pub cursor_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub border_width: u32,
    pub border_color_active: String,
    pub border_color_inactive: String,
    pub gap_inner: u32,
    pub gap_outer: u32,
}

impl Default for Config {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert("Mod4+Return".to_string(), "spawn:gnome-terminal".to_string());
        keybindings.insert("Mod4+q".to_string(), "close".to_string());
        keybindings.insert("Mod4+Shift+e".to_string(), "exit".to_string());
        
        Self {
            general: GeneralConfig {
                focus_follows_mouse: true,
                cursor_theme: "Adwaita".to_string(),
                cursor_size: 24,
            },
            appearance: AppearanceConfig {
                border_width: 2,
                border_color_active: "#88c0d0".to_string(),
                border_color_inactive: "#4c566a".to_string(),
                gap_inner: 8,
                gap_outer: 16,
            },
            keybindings,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config from {:?}", config_path))?;
            
            let config: Config = toml::from_str(&contents)
                .with_context(|| "Failed to parse config file")?;
            
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = toml::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let xdg = BaseDirectories::with_prefix("spinneros")?;
        Ok(xdg.get_config_home().join("spinner-wm.toml"))
    }
}
