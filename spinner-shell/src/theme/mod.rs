//! Theme management

pub struct ThemeManager {
    dark_mode: bool,
    accent_color: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            dark_mode: true,
            accent_color: "#88c0d0".to_string(),
        }
    }
    
    pub fn is_dark_mode(&self) -> bool {
        self.dark_mode
    }
    
    pub fn set_dark_mode(&mut self, enabled: bool) {
        self.dark_mode = enabled;
    }
    
    pub fn accent_color(&self) -> &str {
        &self.accent_color
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
