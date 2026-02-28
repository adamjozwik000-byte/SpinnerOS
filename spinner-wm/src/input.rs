//! Input handling for SpinnerWM

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Shift,
    Control,
    Alt,
    Super,
}

#[derive(Debug, Clone)]
pub enum Action {
    Spawn(String),
    Close,
    Exit,
    Workspace(u32),
    None,
}

pub struct InputHandler {
    keybindings: HashMap<String, Action>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keybindings: HashMap::new(),
        }
    }
    
    pub fn add_keybinding(&mut self, key: String, action: Action) {
        self.keybindings.insert(key, action);
    }
    
    pub fn get_action(&self, key: &str) -> Option<&Action> {
        self.keybindings.get(key)
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
