//! Window management for SpinnerWM

use std::sync::atomic::{AtomicU32, Ordering};

static WINDOW_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u32);

impl WindowId {
    pub fn new() -> Self {
        Self(WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub focused: bool,
}

impl Window {
    pub fn new(title: String, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            id: WindowId::new(),
            title,
            x,
            y,
            width,
            height,
            focused: false,
        }
    }
}

pub struct WindowManager {
    windows: Vec<Window>,
    focused: Option<WindowId>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused: None,
        }
    }
    
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }
    
    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.retain(|w| w.id != id);
    }
    
    pub fn focus_window(&mut self, id: WindowId) {
        self.focused = Some(id);
        for window in &mut self.windows {
            window.focused = window.id == id;
        }
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}
