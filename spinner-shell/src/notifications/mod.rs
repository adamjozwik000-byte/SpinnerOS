//! Notification center

use tracing::info;

pub struct NotificationCenter {
    notifications: Vec<Notification>,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
    
    pub fn add_notification(&mut self, notification: Notification) {
        info!("New notification: {}", notification.summary);
        self.notifications.push(notification);
    }
    
    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}
