//! Input handling for SpinnerWM

use crate::config::Config;
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Shift,
    Control,
    Alt,
    Super,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombination {
    pub modifiers: Vec<Modifier>,
    pub key: String,
}

impl KeyCombination {
    pub fn new(modifiers: Vec<Modifier>, key: String) -> Self {
        Self { modifiers, key }
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('+').collect();
        if parts.is_empty() {
            return None;
        }
        
        let mut modifiers = Vec::new();
        let mut key = String::new();
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                key = part.to_string();
            } else {
                match *part {
                    "Shift" => modifiers.push(Modifier::Shift),
                    "Control" | "Ctrl" => modifiers.push(Modifier::Control),
                    "Alt" | "Mod1" => modifiers.push(Modifier::Alt),
                    "Super" | "Mod4" => modifiers.push(Modifier::Super),
                    _ => {}
                }
            }
        }
        
        Some(Self { modifiers, key })
    }
    
    pub fn matches(&self, modifiers: &[Modifier], key: &str) -> bool {
        if self.key != key {
            return false;
        }
        
        for m in &self.modifiers {
            if !modifiers.contains(m) {
                return false;
            }
        }
        
        self.modifiers.len() == modifiers.len()
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Spawn(String),
    Close,
    Fullscreen,
    ToggleFloating,
    Maximize,
    Minimize,
    Exit,
    Workspace(u32),
    MoveToWorkspace(u32),
    Focus(Direction),
    Move(Direction),
    Resize(Direction, i32),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "left" => Some(Direction::Left),
            "right" => Some(Direction::Right),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None,
        }
    }
}

pub struct InputHandler {
    keybindings: HashMap<KeyCombination, Action>,
    current_modifiers: Vec<Modifier>,
}

impl InputHandler {
    pub fn new(config: &Config) -> Self {
        let mut keybindings = HashMap::new();
        
        for (key_str, action_str) in &config.keybindings {
            if let Some(key_combo) = KeyCombination::from_string(key_str) {
                let action = Self::parse_action(action_str);
                keybindings.insert(key_combo, action);
            }
        }
        
        info!("Loaded {} keybindings", keybindings.len());
        
        Self {
            keybindings,
            current_modifiers: Vec::new(),
        }
    }
    
    fn parse_action(action_str: &str) -> Action {
        if let Some(cmd) = action_str.strip_prefix("spawn:") {
            return Action::Spawn(cmd.to_string());
        }
        
        if let Some(num) = action_str.strip_prefix("workspace:") {
            if let Ok(n) = num.parse() {
                return Action::Workspace(n);
            }
        }
        
        if let Some(num) = action_str.strip_prefix("move_to_workspace:") {
            if let Ok(n) = num.parse() {
                return Action::MoveToWorkspace(n);
            }
        }
        
        if let Some(dir) = action_str.strip_prefix("focus:") {
            if let Some(d) = Direction::from_string(dir) {
                return Action::Focus(d);
            }
        }
        
        if let Some(dir) = action_str.strip_prefix("move:") {
            if let Some(d) = Direction::from_string(dir) {
                return Action::Move(d);
            }
        }
        
        match action_str {
            "close" => Action::Close,
            "fullscreen" => Action::Fullscreen,
            "toggle_floating" => Action::ToggleFloating,
            "maximize" => Action::Maximize,
            "minimize" => Action::Minimize,
            "exit" => Action::Exit,
            _ => Action::None,
        }
    }
    
    pub fn modifier_pressed(&mut self, modifier: Modifier) {
        if !self.current_modifiers.contains(&modifier) {
            self.current_modifiers.push(modifier);
        }
    }
    
    pub fn modifier_released(&mut self, modifier: Modifier) {
        self.current_modifiers.retain(|m| *m != modifier);
    }
    
    pub fn key_pressed(&self, key: &str) -> Option<Action> {
        debug!("Key pressed: {} with modifiers: {:?}", key, self.current_modifiers);
        
        for (combo, action) in &self.keybindings {
            if combo.matches(&self.current_modifiers, key) {
                return Some(action.clone());
            }
        }
        
        None
    }
    
    pub fn current_modifiers(&self) -> &[Modifier] {
        &self.current_modifiers
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub x: f64,
    pub y: f64,
    pub button_left: bool,
    pub button_right: bool,
    pub button_middle: bool,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            button_left: false,
            button_right: false,
            button_middle: false,
        }
    }
}

impl MouseState {
    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    
    pub fn update_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    
    pub fn any_button_pressed(&self) -> bool {
        self.button_left || self.button_right || self.button_middle
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DragOperation {
    None,
    Move { start_x: f64, start_y: f64, window_x: i32, window_y: i32 },
    Resize { start_x: f64, start_y: f64, original_width: u32, original_height: u32 },
}

impl Default for DragOperation {
    fn default() -> Self {
        Self::None
    }
}
