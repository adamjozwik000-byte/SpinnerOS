//! Application launcher

use super::AppEntry;

pub struct AppLauncher {
    apps: Vec<AppEntry>,
}

impl AppLauncher {
    pub fn new() -> Self {
        Self {
            apps: vec![
                AppEntry {
                    name: "Files".to_string(),
                    exec: "nautilus".to_string(),
                    icon: "system-file-manager".to_string(),
                    description: "Browse files".to_string(),
                    keywords: vec!["file".to_string()],
                },
                AppEntry {
                    name: "Firefox".to_string(),
                    exec: "firefox".to_string(),
                    icon: "firefox".to_string(),
                    description: "Web Browser".to_string(),
                    keywords: vec!["web".to_string()],
                },
            ],
        }
    }
    
    pub fn get_apps(&self) -> &[AppEntry] {
        &self.apps
    }
}

impl Default for AppLauncher {
    fn default() -> Self {
        Self::new()
    }
}
