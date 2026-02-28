//! Window management for SpinnerWM

use smithay::utils::{Logical, Point, Rectangle, Size};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Maximized,
    Fullscreen,
    Minimized,
}

#[derive(Debug, Clone)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowGeometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn position(&self) -> Point<i32, Logical> {
        Point::from((self.x, self.y))
    }
    
    pub fn size(&self) -> Size<i32, Logical> {
        Size::from((self.width as i32, self.height as i32))
    }
    
    pub fn to_rectangle(&self) -> Rectangle<i32, Logical> {
        Rectangle::from_loc_and_size(self.position(), self.size())
    }
    
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x 
            && x < self.x + self.width as i32 
            && y >= self.y 
            && y < self.y + self.height as i32
    }
}

#[derive(Debug, Clone)]
pub struct ManagedWindow {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub geometry: WindowGeometry,
    pub saved_geometry: Option<WindowGeometry>,
    pub state: WindowState,
    pub is_floating: bool,
    pub workspace: u32,
    pub focused: bool,
    pub urgent: bool,
    pub decorations: bool,
}

impl ManagedWindow {
    pub fn new(title: String, app_id: String, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            id: WindowId::new(),
            title,
            app_id,
            geometry: WindowGeometry::new(x, y, width, height),
            saved_geometry: None,
            state: WindowState::Normal,
            is_floating: true,
            workspace: 1,
            focused: false,
            urgent: false,
            decorations: true,
        }
    }
    
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }
    
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.geometry.width = width;
        self.geometry.height = height;
    }
    
    pub fn maximize(&mut self, screen_width: u32, screen_height: u32, panel_height: u32) {
        if self.state != WindowState::Maximized {
            self.saved_geometry = Some(self.geometry.clone());
            self.geometry = WindowGeometry::new(
                0,
                panel_height as i32,
                screen_width,
                screen_height - panel_height,
            );
            self.state = WindowState::Maximized;
        }
    }
    
    pub fn unmaximize(&mut self) {
        if self.state == WindowState::Maximized {
            if let Some(saved) = self.saved_geometry.take() {
                self.geometry = saved;
            }
            self.state = WindowState::Normal;
        }
    }
    
    pub fn toggle_maximize(&mut self, screen_width: u32, screen_height: u32, panel_height: u32) {
        if self.state == WindowState::Maximized {
            self.unmaximize();
        } else {
            self.maximize(screen_width, screen_height, panel_height);
        }
    }
    
    pub fn fullscreen(&mut self, screen_width: u32, screen_height: u32) {
        if self.state != WindowState::Fullscreen {
            self.saved_geometry = Some(self.geometry.clone());
            self.geometry = WindowGeometry::new(0, 0, screen_width, screen_height);
            self.state = WindowState::Fullscreen;
            self.decorations = false;
        }
    }
    
    pub fn unfullscreen(&mut self) {
        if self.state == WindowState::Fullscreen {
            if let Some(saved) = self.saved_geometry.take() {
                self.geometry = saved;
            }
            self.state = WindowState::Normal;
            self.decorations = true;
        }
    }
    
    pub fn toggle_fullscreen(&mut self, screen_width: u32, screen_height: u32) {
        if self.state == WindowState::Fullscreen {
            self.unfullscreen();
        } else {
            self.fullscreen(screen_width, screen_height);
        }
    }
    
    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
    }
    
    pub fn restore(&mut self) {
        if self.state == WindowState::Minimized {
            self.state = WindowState::Normal;
        }
    }
    
    pub fn toggle_floating(&mut self) {
        self.is_floating = !self.is_floating;
    }
    
    pub fn is_visible(&self) -> bool {
        self.state != WindowState::Minimized
    }
}

#[derive(Debug)]
pub struct WindowManager {
    windows: Vec<ManagedWindow>,
    focused_window: Option<WindowId>,
    current_workspace: u32,
    screen_width: u32,
    screen_height: u32,
    panel_height: u32,
}

impl WindowManager {
    pub fn new(screen_width: u32, screen_height: u32, panel_height: u32) -> Self {
        Self {
            windows: Vec::new(),
            focused_window: None,
            current_workspace: 1,
            screen_width,
            screen_height,
            panel_height,
        }
    }
    
    pub fn add_window(&mut self, mut window: ManagedWindow) {
        window.workspace = self.current_workspace;
        
        self.center_window(&mut window);
        
        self.windows.push(window.clone());
        self.focus_window(window.id);
    }
    
    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.retain(|w| w.id != id);
        
        if self.focused_window == Some(id) {
            self.focused_window = self.visible_windows().last().map(|w| w.id);
        }
    }
    
    pub fn get_window(&self, id: WindowId) -> Option<&ManagedWindow> {
        self.windows.iter().find(|w| w.id == id)
    }
    
    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut ManagedWindow> {
        self.windows.iter_mut().find(|w| w.id == id)
    }
    
    pub fn focus_window(&mut self, id: WindowId) {
        for window in &mut self.windows {
            window.focused = window.id == id;
        }
        self.focused_window = Some(id);
        
        if let Some(idx) = self.windows.iter().position(|w| w.id == id) {
            let window = self.windows.remove(idx);
            self.windows.push(window);
        }
    }
    
    pub fn focused_window(&self) -> Option<&ManagedWindow> {
        self.focused_window.and_then(|id| self.get_window(id))
    }
    
    pub fn focused_window_mut(&mut self) -> Option<&mut ManagedWindow> {
        let id = self.focused_window?;
        self.get_window_mut(id)
    }
    
    pub fn visible_windows(&self) -> impl Iterator<Item = &ManagedWindow> {
        self.windows.iter().filter(move |w| {
            w.workspace == self.current_workspace && w.is_visible()
        })
    }
    
    pub fn all_windows(&self) -> &[ManagedWindow] {
        &self.windows
    }
    
    pub fn switch_workspace(&mut self, workspace: u32) {
        self.current_workspace = workspace;
        
        self.focused_window = self
            .visible_windows()
            .last()
            .map(|w| w.id);
    }
    
    pub fn move_window_to_workspace(&mut self, id: WindowId, workspace: u32) {
        if let Some(window) = self.get_window_mut(id) {
            window.workspace = workspace;
        }
    }
    
    pub fn current_workspace(&self) -> u32 {
        self.current_workspace
    }
    
    pub fn window_at_point(&self, x: i32, y: i32) -> Option<&ManagedWindow> {
        self.visible_windows()
            .rev()
            .find(|w| w.geometry.contains_point(x, y))
    }
    
    fn center_window(&self, window: &mut ManagedWindow) {
        let available_height = self.screen_height - self.panel_height;
        window.geometry.x = ((self.screen_width - window.geometry.width) / 2) as i32;
        window.geometry.y = self.panel_height as i32 + ((available_height - window.geometry.height) / 2) as i32;
    }
    
    pub fn update_screen_size(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }
}
