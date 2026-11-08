//! Terminal manager for alacritty.

use std::collections::HashMap;
use crate::terminal::{Terminal, TerminalState, TerminalConfig};
use crate::session::{Session, SessionType, SessionStatus};
use crate::renderer::{Renderer, RenderBackend};
use crate::input::{InputHandler, KeyBinding, KeyAction};
use crate::config::{Theme, FontConfig, ColorScheme};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max terminals
    pub max_terminals: usize,
    /// Max sessions
    pub max_sessions: usize,
    /// Default terminal config
    pub default_terminal_config: TerminalConfig,
    /// Default render backend
    pub default_render_backend: RenderBackend,
    /// Enable GPU acceleration
    pub enable_gpu_acceleration: bool,
    /// Enable mouse support
    pub enable_mouse_support: bool,
    /// Enable copy on select
    pub enable_copy_on_select: bool,
    /// Enable right click paste
    pub enable_right_click_paste: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_terminals: 10,
            max_sessions: 100,
            default_terminal_config: TerminalConfig::default(),
            default_render_backend: RenderBackend::Auto,
            enable_gpu_acceleration: true,
            enable_mouse_support: true,
            enable_copy_on_select: true,
            enable_right_click_paste: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total terminals
    pub total_terminals: usize,
    /// Total sessions
    pub total_sessions: usize,
    /// Active terminals
    pub active_terminals: usize,
    /// Active sessions
    pub active_sessions: usize,
    /// Running terminals
    pub running_terminals: usize,
    /// Running sessions
    pub running_sessions: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Terminal manager
#[derive(Debug, Clone)]
pub struct TerminalManager {
    /// Terminals
    terminals: HashMap<String, Terminal>,
    /// Sessions
    sessions: HashMap<String, Session>,
    /// Renderers
    renderers: HashMap<String, Renderer>,
    /// Input handlers
    input_handlers: HashMap<String, InputHandler>,
    /// Themes
    themes: HashMap<String, Theme>,
    /// Font configs
    font_configs: HashMap<String, FontConfig>,
    /// Color schemes
    color_schemes: HashMap<String, ColorScheme>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self {
            terminals: HashMap::new(),
            sessions: HashMap::new(),
            renderers: HashMap::new(),
            input_handlers: HashMap::new(),
            themes: HashMap::new(),
            font_configs: HashMap::new(),
            color_schemes: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_terminals: 0,
                total_sessions: 0,
                active_terminals: 0,
                active_sessions: 0,
                running_terminals: 0,
                running_sessions: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl TerminalManager {
    /// Create new terminal manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            terminals: HashMap::new(),
            sessions: HashMap::new(),
            renderers: HashMap::new(),
            input_handlers: HashMap::new(),
            themes: HashMap::new(),
            font_configs: HashMap::new(),
            color_schemes: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_terminals: 0,
                total_sessions: 0,
                active_terminals: 0,
                active_sessions: 0,
                running_terminals: 0,
                running_sessions: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add default theme
        let default_theme = Theme::new("default".into());
        self.themes.insert("default".into(), default_theme);
        
        // Add default font config
        let default_font = FontConfig::new("Monospace".into(), 12);
        self.font_configs.insert("default".into(), default_font);
        
        // Add default color scheme
        let default_scheme = ColorScheme::new();
        self.color_schemes.insert("default".into(), default_scheme);
        
        self.update_statistics();
    }
    
    // Terminal methods
    
    /// Create terminal
    pub fn create_terminal(&mut self) -> String {
        let id = format!("term-{}", self.terminals.len() + 1);
        let terminal = Terminal::new(id.clone());
        
        self.terminals.insert(id.clone(), terminal);
        self.update_statistics();
        id
    }
    
    /// Get terminal
    pub fn get_terminal(&self, terminal_id: &str) -> Option<&Terminal> {
        self.terminals.get(terminal_id)
    }
    
    /// Get terminal mutably
    pub fn get_terminal_mut(&mut self, terminal_id: &str) -> Option<&mut Terminal> {
        self.terminals.get_mut(terminal_id)
    }
    
    /// Get all terminals
    pub fn list_terminals(&self) -> Vec<&Terminal> {
        self.terminals.values().collect()
    }
    
    /// Delete terminal
    pub fn delete_terminal(&mut self, terminal_id: &str) -> bool {
        if self.terminals.remove(terminal_id).is_some() {
            // Remove associated renderer and input handler
            self.renderers.remove(terminal_id);
            self.input_handlers.remove(terminal_id);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start terminal
    pub fn start_terminal(&mut self, terminal_id: &str) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            if terminal.start() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Stop terminal
    pub fn stop_terminal(&mut self, terminal_id: &str) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            if terminal.stop() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Set terminal config
    pub fn set_terminal_config(&mut self, terminal_id: &str, config: TerminalConfig) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            terminal.set_config(config);
            return true;
        }
        false
    }
    
    /// Set terminal title
    pub fn set_terminal_title(&mut self, terminal_id: &str, title: String) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            terminal.set_title(title);
            return true;
        }
        false
    }
    
    /// Set terminal working directory
    pub fn set_terminal_working_directory(&mut self, terminal_id: &str, directory: String) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            terminal.set_working_directory(directory);
            return true;
        }
        false
    }
    
    /// Set terminal command
    pub fn set_terminal_command(&mut self, terminal_id: &str, command: String) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            terminal.set_command(command);
            return true;
        }
        false
    }
    
    /// Set terminal size
    pub fn set_terminal_size(&mut self, terminal_id: &str, width: u32, height: u32) -> bool {
        if let Some(terminal) = self.terminals.get_mut(terminal_id) {
            terminal.set_size(width, height);
            return true;
        }
        false
    }
    
    /// Get terminals by state
    pub fn get_terminals_by_state(&self, state: &TerminalState) -> Vec<&Terminal> {
        self.terminals.values()
            .filter(|t| t.state == *state)
            .collect()
    }
    
    /// Get running terminals
    pub fn get_running_terminals(&self) -> Vec<&Terminal> {
        self.get_terminals_by_state(&TerminalState::Running)
    }
    
    /// Get idle terminals
    pub fn get_idle_terminals(&self) -> Vec<&Terminal> {
        self.get_terminals_by_state(&TerminalState::Idle)
    }
    
    // Session methods
    
    /// Create session
    pub fn create_session(&mut self, session_type: SessionType) -> String {
        let id = format!("sess-{}", self.sessions.len() + 1);
        let session = Session::new(id.clone(), session_type);
        
        self.sessions.insert(id.clone(), session);
        self.update_statistics();
        id
    }
    
    /// Get session
    pub fn get_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }
    
    /// Get session mutably
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }
    
    /// Get all sessions
    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.values().collect()
    }
    
    /// Delete session
    pub fn delete_session(&mut self, session_id: &str) -> bool {
        if self.sessions.remove(session_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Close session
    pub fn close_session(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if session.stop() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Set session config
    pub fn set_session_config(&mut self, session_id: &str, config: crate::session::SessionConfig) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.set_config(config);
            return true;
        }
        false
    }
    
    /// Set session title
    pub fn set_session_title(&mut self, session_id: &str, title: String) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.set_title(title);
            return true;
        }
        false
    }
    
    /// Get sessions by type
    pub fn get_sessions_by_type(&self, session_type: &SessionType) -> Vec<&Session> {
        self.sessions.values()
            .filter(|s| s.session_type == *session_type)
            .collect()
    }
    
    /// Get sessions by status
    pub fn get_sessions_by_status(&self, status: &SessionStatus) -> Vec<&Session> {
        self.sessions.values()
            .filter(|s| s.status == *status)
            .collect()
    }
    
    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<&Session> {
        self.get_sessions_by_status(&SessionStatus::Active)
    }
    
    /// Get exited sessions
    pub fn get_exited_sessions(&self) -> Vec<&Session> {
        self.get_sessions_by_status(&SessionStatus::Exited)
    }
    
    // Renderer methods
    
    /// Create renderer for terminal
    pub fn create_renderer(&mut self, _terminal_id: &str, backend: RenderBackend) -> String {
        let id = format!("renderer-{}", self.renderers.len() + 1);
        let renderer = Renderer::new(backend);
        
        self.renderers.insert(id.clone(), renderer);
        id
    }
    
    /// Get renderer
    pub fn get_renderer(&self, renderer_id: &str) -> Option<&Renderer> {
        self.renderers.get(renderer_id)
    }
    
    /// Get renderer mutably
    pub fn get_renderer_mut(&mut self, renderer_id: &str) -> Option<&mut Renderer> {
        self.renderers.get_mut(renderer_id)
    }
    
    /// Get all renderers
    pub fn list_renderers(&self) -> Vec<&Renderer> {
        self.renderers.values().collect()
    }
    
    /// Delete renderer
    pub fn delete_renderer(&mut self, renderer_id: &str) -> bool {
        self.renderers.remove(renderer_id).is_some()
    }
    
    /// Start renderer
    pub fn start_renderer(&mut self, renderer_id: &str) -> bool {
        if let Some(renderer) = self.renderers.get_mut(renderer_id) {
            if renderer.start() {
                return true;
            }
        }
        false
    }
    
    /// Stop renderer
    pub fn stop_renderer(&mut self, renderer_id: &str) -> bool {
        if let Some(renderer) = self.renderers.get_mut(renderer_id) {
            if renderer.stop() {
                return true;
            }
        }
        false
    }
    
    /// Render frame
    pub fn render_frame(&mut self, renderer_id: &str) -> bool {
        if let Some(renderer) = self.renderers.get_mut(renderer_id) {
            return renderer.render_frame();
        }
        false
    }
    
    // Input handler methods
    
    /// Create input handler for terminal
    pub fn create_input_handler(&mut self, _terminal_id: &str) -> String {
        let id = format!("input-{}", self.input_handlers.len() + 1);
        let handler = InputHandler::new();
        
        self.input_handlers.insert(id.clone(), handler);
        id
    }
    
    /// Get input handler
    pub fn get_input_handler(&self, handler_id: &str) -> Option<&InputHandler> {
        self.input_handlers.get(handler_id)
    }
    
    /// Get input handler mutably
    pub fn get_input_handler_mut(&mut self, handler_id: &str) -> Option<&mut InputHandler> {
        self.input_handlers.get_mut(handler_id)
    }
    
    /// Get all input handlers
    pub fn list_input_handlers(&self) -> Vec<&InputHandler> {
        self.input_handlers.values().collect()
    }
    
    /// Delete input handler
    pub fn delete_input_handler(&mut self, handler_id: &str) -> bool {
        self.input_handlers.remove(handler_id).is_some()
    }
    
    /// Add key binding
    pub fn add_key_binding(&mut self, handler_id: &str, binding: KeyBinding) -> bool {
        if let Some(handler) = self.input_handlers.get_mut(handler_id) {
            handler.add_key_binding(binding);
            return true;
        }
        false
    }
    
    /// Handle input event
    pub fn handle_input_event(&mut self, handler_id: &str, event: crate::input::InputEvent) -> Option<KeyAction> {
        if let Some(handler) = self.input_handlers.get_mut(handler_id) {
            return handler.handle_event(event);
        }
        None
    }
    
    // Theme methods
    
    /// Add theme
    pub fn add_theme(&mut self, name: String, theme: Theme) -> bool {
        self.themes.insert(name, theme);
        true
    }
    
    /// Get theme
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
    
    /// Get theme mutably
    pub fn get_theme_mut(&mut self, name: &str) -> Option<&mut Theme> {
        self.themes.get_mut(name)
    }
    
    /// Get all themes
    pub fn list_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }
    
    /// Delete theme
    pub fn delete_theme(&mut self, name: &str) -> bool {
        self.themes.remove(name).is_some()
    }
    
    // Font config methods
    
    /// Add font config
    pub fn add_font_config(&mut self, name: String, font: FontConfig) -> bool {
        self.font_configs.insert(name, font);
        true
    }
    
    /// Get font config
    pub fn get_font_config(&self, name: &str) -> Option<&FontConfig> {
        self.font_configs.get(name)
    }
    
    /// Get font config mutably
    pub fn get_font_config_mut(&mut self, name: &str) -> Option<&mut FontConfig> {
        self.font_configs.get_mut(name)
    }
    
    /// Get all font configs
    pub fn list_font_configs(&self) -> Vec<&FontConfig> {
        self.font_configs.values().collect()
    }
    
    /// Delete font config
    pub fn delete_font_config(&mut self, name: &str) -> bool {
        self.font_configs.remove(name).is_some()
    }
    
    // Color scheme methods
    
    /// Add color scheme
    pub fn add_color_scheme(&mut self, name: String, scheme: ColorScheme) -> bool {
        self.color_schemes.insert(name, scheme);
        true
    }
    
    /// Get color scheme
    pub fn get_color_scheme(&self, name: &str) -> Option<&ColorScheme> {
        self.color_schemes.get(name)
    }
    
    /// Get color scheme mutably
    pub fn get_color_scheme_mut(&mut self, name: &str) -> Option<&mut ColorScheme> {
        self.color_schemes.get_mut(name)
    }
    
    /// Get all color schemes
    pub fn list_color_schemes(&self) -> Vec<&ColorScheme> {
        self.color_schemes.values().collect()
    }
    
    /// Delete color scheme
    pub fn delete_color_scheme(&mut self, name: &str) -> bool {
        self.color_schemes.remove(name).is_some()
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_terminals = self.terminals.len();
        self.statistics.total_sessions = self.sessions.len();
        self.statistics.active_terminals = self.terminals.values()
            .filter(|t| t.state != TerminalState::Exited)
            .count();
        self.statistics.active_sessions = self.sessions.values()
            .filter(|s| s.status != SessionStatus::Exited)
            .count();
        self.statistics.running_terminals = self.terminals.values()
            .filter(|t| t.state == TerminalState::Running)
            .count();
        self.statistics.running_sessions = self.sessions.values()
            .filter(|s| s.status == SessionStatus::Active)
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            terminals: self.terminals.values().cloned().collect(),
            sessions: self.sessions.values().cloned().collect(),
            themes: self.themes.values().cloned().collect(),
            font_configs: self.font_configs.values().cloned().collect(),
            color_schemes: self.color_schemes.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.terminals.clear();
        for terminal in export.terminals {
            self.terminals.insert(terminal.id.clone(), terminal);
        }
        
        self.sessions.clear();
        for session in export.sessions {
            self.sessions.insert(session.id.clone(), session);
        }
        
        self.themes.clear();
        for theme in export.themes {
            self.themes.insert(theme.name.clone(), theme);
        }
        
        self.font_configs.clear();
        for font in export.font_configs {
            self.font_configs.insert(font.family.clone(), font);
        }
        
        self.color_schemes.clear();
        for scheme in export.color_schemes {
            self.color_schemes.insert(scheme.name.clone(), scheme);
        }
        
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Terminals
    pub terminals: Vec<Terminal>,
    /// Sessions
    pub sessions: Vec<Session>,
    /// Themes
    pub themes: Vec<Theme>,
    /// Font configs
    pub font_configs: Vec<FontConfig>,
    /// Color schemes
    pub color_schemes: Vec<ColorScheme>,
}
