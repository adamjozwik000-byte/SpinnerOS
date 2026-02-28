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
    pub workspaces: WorkspaceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub focus_follows_mouse: bool,
    pub cursor_theme: String,
    pub cursor_size: u32,
    pub autostart: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub border_width: u32,
    pub border_color_active: String,
    pub border_color_inactive: String,
    pub gap_inner: u32,
    pub gap_outer: u32,
    pub animation_duration_ms: u32,
    pub enable_transparency: bool,
    pub blur_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub count: u32,
    pub names: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        
        keybindings.insert("Mod4+Return".to_string(), "spawn:spinner-terminal".to_string());
        keybindings.insert("Mod4+d".to_string(), "spawn:spinner-shell --menu".to_string());
        keybindings.insert("Mod4+q".to_string(), "close".to_string());
        keybindings.insert("Mod4+f".to_string(), "fullscreen".to_string());
        keybindings.insert("Mod4+space".to_string(), "toggle_floating".to_string());
        keybindings.insert("Mod4+Shift+e".to_string(), "exit".to_string());
        
        keybindings.insert("Mod4+1".to_string(), "workspace:1".to_string());
        keybindings.insert("Mod4+2".to_string(), "workspace:2".to_string());
        keybindings.insert("Mod4+3".to_string(), "workspace:3".to_string());
        keybindings.insert("Mod4+4".to_string(), "workspace:4".to_string());
        keybindings.insert("Mod4+5".to_string(), "workspace:5".to_string());
        
        keybindings.insert("Mod4+Shift+1".to_string(), "move_to_workspace:1".to_string());
        keybindings.insert("Mod4+Shift+2".to_string(), "move_to_workspace:2".to_string());
        keybindings.insert("Mod4+Shift+3".to_string(), "move_to_workspace:3".to_string());
        keybindings.insert("Mod4+Shift+4".to_string(), "move_to_workspace:4".to_string());
        keybindings.insert("Mod4+Shift+5".to_string(), "move_to_workspace:5".to_string());
        
        keybindings.insert("Mod4+Left".to_string(), "focus:left".to_string());
        keybindings.insert("Mod4+Right".to_string(), "focus:right".to_string());
        keybindings.insert("Mod4+Up".to_string(), "focus:up".to_string());
        keybindings.insert("Mod4+Down".to_string(), "focus:down".to_string());
        
        Self {
            general: GeneralConfig {
                focus_follows_mouse: true,
                cursor_theme: "Adwaita".to_string(),
                cursor_size: 24,
                autostart: vec![
                    "spinner-shell".to_string(),
                    "pipewire".to_string(),
                    "wireplumber".to_string(),
                ],
            },
            appearance: AppearanceConfig {
                border_width: 2,
                border_color_active: "#88c0d0".to_string(),
                border_color_inactive: "#4c566a".to_string(),
                gap_inner: 8,
                gap_outer: 16,
                animation_duration_ms: 200,
                enable_transparency: true,
                blur_strength: 0.5,
            },
            keybindings,
            workspaces: WorkspaceConfig {
                count: 5,
                names: vec![
                    "Main".to_string(),
                    "Web".to_string(),
                    "Code".to_string(),
                    "Media".to_string(),
                    "Other".to_string(),
                ],
            },
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
    
    pub fn get_keybinding_action(&self, key: &str) -> Option<&String> {
        self.keybindings.get(key)
    }
}
