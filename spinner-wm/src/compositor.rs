//! Wayland compositor implementation for SpinnerWM

use crate::config::Config;
use crate::input::{Action, DragOperation, InputHandler, MouseState};
use crate::window::{ManagedWindow, WindowId, WindowManager};

use anyhow::{Context, Result};
use calloop::{EventLoop, LoopSignal};
use std::process::Command;
use std::time::Duration;
use tracing::{debug, error, info, warn};

pub struct SpinnerCompositor {
    config: Config,
    window_manager: WindowManager,
    input_handler: InputHandler,
    mouse_state: MouseState,
    drag_operation: DragOperation,
    running: bool,
    loop_signal: Option<LoopSignal>,
}

impl SpinnerCompositor {
    pub fn new(config: Config) -> Result<Self> {
        let screen_width = 1920;
        let screen_height = 1080;
        let panel_height = 48;
        
        let window_manager = WindowManager::new(screen_width, screen_height, panel_height);
        let input_handler = InputHandler::new(&config);
        
        Ok(Self {
            config,
            window_manager,
            input_handler,
            mouse_state: MouseState::default(),
            drag_operation: DragOperation::None,
            running: true,
            loop_signal: None,
        })
    }
    
    pub fn run(&mut self) -> Result<()> {
        info!("Starting SpinnerWM event loop");
        
        self.run_autostart()?;
        
        let mut event_loop: EventLoop<Self> = EventLoop::try_new()
            .context("Failed to create event loop")?;
        
        self.loop_signal = Some(event_loop.get_signal());
        
        info!("SpinnerWM is running. Press Mod4+Shift+E to exit.");
        
        while self.running {
            event_loop
                .dispatch(Duration::from_millis(16), self)
                .context("Event loop dispatch failed")?;
            
            self.process_frame();
        }
        
        info!("Event loop ended");
        Ok(())
    }
    
    fn run_autostart(&self) -> Result<()> {
        info!("Running autostart applications");
        
        for cmd in &self.config.general.autostart {
            info!("Starting: {}", cmd);
            
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match Command::new(parts[0]).args(&parts[1..]).spawn() {
                Ok(_) => debug!("Started: {}", cmd),
                Err(e) => warn!("Failed to start {}: {}", cmd, e),
            }
        }
        
        Ok(())
    }
    
    fn process_frame(&mut self) {
        // Frame processing - animations, damage tracking, etc.
    }
    
    pub fn handle_key_press(&mut self, key: &str) {
        if let Some(action) = self.input_handler.key_pressed(key) {
            self.execute_action(action);
        }
    }
    
    pub fn handle_mouse_motion(&mut self, x: f64, y: f64) {
        self.mouse_state.update_position(x, y);
        
        match self.drag_operation {
            DragOperation::Move { start_x, start_y, window_x, window_y } => {
                let dx = (x - start_x) as i32;
                let dy = (y - start_y) as i32;
                
                if let Some(window) = self.window_manager.focused_window_mut() {
                    window.set_position(window_x + dx, window_y + dy);
                }
            }
            DragOperation::Resize { start_x, start_y, original_width, original_height } => {
                let dx = (x - start_x) as i32;
                let dy = (y - start_y) as i32;
                
                let new_width = (original_width as i32 + dx).max(100) as u32;
                let new_height = (original_height as i32 + dy).max(100) as u32;
                
                if let Some(window) = self.window_manager.focused_window_mut() {
                    window.set_size(new_width, new_height);
                }
            }
            DragOperation::None => {
                if self.config.general.focus_follows_mouse {
                    if let Some(window) = self.window_manager.window_at_point(x as i32, y as i32) {
                        let id = window.id;
                        self.window_manager.focus_window(id);
                    }
                }
            }
        }
    }
    
    pub fn handle_mouse_button(&mut self, button: u32, pressed: bool) {
        match button {
            0x110 => self.mouse_state.button_left = pressed,
            0x111 => self.mouse_state.button_right = pressed,
            0x112 => self.mouse_state.button_middle = pressed,
            _ => {}
        }
        
        if pressed {
            let modifiers = self.input_handler.current_modifiers();
            let has_super = modifiers.iter().any(|m| matches!(m, crate::input::Modifier::Super));
            
            if has_super && button == 0x110 {
                if let Some(window) = self.window_manager.focused_window() {
                    self.drag_operation = DragOperation::Move {
                        start_x: self.mouse_state.x,
                        start_y: self.mouse_state.y,
                        window_x: window.geometry.x,
                        window_y: window.geometry.y,
                    };
                }
            } else if has_super && button == 0x111 {
                if let Some(window) = self.window_manager.focused_window() {
                    self.drag_operation = DragOperation::Resize {
                        start_x: self.mouse_state.x,
                        start_y: self.mouse_state.y,
                        original_width: window.geometry.width,
                        original_height: window.geometry.height,
                    };
                }
            } else if button == 0x110 {
                if let Some(window) = self.window_manager.window_at_point(
                    self.mouse_state.x as i32,
                    self.mouse_state.y as i32,
                ) {
                    let id = window.id;
                    self.window_manager.focus_window(id);
                }
            }
        } else {
            self.drag_operation = DragOperation::None;
        }
    }
    
    fn execute_action(&mut self, action: Action) {
        match action {
            Action::Spawn(cmd) => {
                self.spawn_command(&cmd);
            }
            Action::Close => {
                if let Some(window) = self.window_manager.focused_window() {
                    let id = window.id;
                    self.close_window(id);
                }
            }
            Action::Fullscreen => {
                if let Some(window) = self.window_manager.focused_window_mut() {
                    window.toggle_fullscreen(1920, 1080);
                }
            }
            Action::ToggleFloating => {
                if let Some(window) = self.window_manager.focused_window_mut() {
                    window.toggle_floating();
                }
            }
            Action::Maximize => {
                if let Some(window) = self.window_manager.focused_window_mut() {
                    window.toggle_maximize(1920, 1080, 48);
                }
            }
            Action::Minimize => {
                if let Some(window) = self.window_manager.focused_window_mut() {
                    window.minimize();
                }
            }
            Action::Exit => {
                info!("Exit requested");
                self.running = false;
                if let Some(signal) = &self.loop_signal {
                    signal.stop();
                }
            }
            Action::Workspace(n) => {
                self.window_manager.switch_workspace(n);
                info!("Switched to workspace {}", n);
            }
            Action::MoveToWorkspace(n) => {
                if let Some(window) = self.window_manager.focused_window() {
                    let id = window.id;
                    self.window_manager.move_window_to_workspace(id, n);
                    info!("Moved window to workspace {}", n);
                }
            }
            Action::Focus(_direction) => {
                debug!("Focus direction not yet implemented");
            }
            Action::Move(_direction) => {
                debug!("Move direction not yet implemented");
            }
            Action::Resize(_direction, _amount) => {
                debug!("Resize direction not yet implemented");
            }
            Action::None => {}
        }
    }
    
    fn spawn_command(&self, cmd: &str) {
        info!("Spawning: {}", cmd);
        
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        
        match Command::new(parts[0]).args(&parts[1..]).spawn() {
            Ok(_) => debug!("Spawned: {}", cmd),
            Err(e) => error!("Failed to spawn {}: {}", cmd, e),
        }
    }
    
    fn close_window(&mut self, id: WindowId) {
        info!("Closing window {:?}", id);
        self.window_manager.remove_window(id);
    }
    
    pub fn add_window(&mut self, title: String, app_id: String, width: u32, height: u32) {
        let window = ManagedWindow::new(title, app_id, 0, 0, width, height);
        self.window_manager.add_window(window);
    }
    
    pub fn window_manager(&self) -> &WindowManager {
        &self.window_manager
    }
    
    pub fn window_manager_mut(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
}
