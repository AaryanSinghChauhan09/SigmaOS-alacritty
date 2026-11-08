//! Terminal definitions for alacritty.

/// Terminal mode
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalMode {
    /// Normal mode
    Normal,
    /// Alternate screen mode
    Alternate,
    /// Scrollback mode
    Scrollback,
    /// Selection mode
    Selection,
    /// Search mode
    Search,
}

/// Terminal state
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalState {
    /// Idle state
    Idle,
    /// Running state
    Running,
    /// Paused state
    Paused,
    /// Exited state
    Exited,
    /// Error state
    Error,
}

/// Terminal config
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    /// Font size
    pub font_size: u32,
    /// Font family
    pub font_family: String,
    /// Theme
    pub theme: String,
    /// Opacity
    pub opacity: f64,
    /// Scrollback lines
    pub scrollback_lines: usize,
    /// Cursor style
    pub cursor_style: CursorStyle,
    /// Cursor blink
    pub cursor_blink: bool,
    /// Window decorations
    pub window_decorations: bool,
    /// Window opacity
    pub window_opacity: f64,
    /// Background opacity
    pub background_opacity: f64,
}

/// Cursor style
#[derive(Debug, Clone, PartialEq)]
pub enum CursorStyle {
    /// Block cursor
    Block,
    /// Underline cursor
    Underline,
    /// Bar cursor
    Bar,
    /// Hollow block cursor
    HollowBlock,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            font_size: 12,
            font_family: "Monospace".into(),
            theme: "default".into(),
            opacity: 1.0,
            scrollback_lines: 10000,
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            window_decorations: true,
            window_opacity: 1.0,
            background_opacity: 1.0,
        }
    }
}

/// Terminal
#[derive(Debug, Clone)]
pub struct Terminal {
    /// Terminal ID
    pub id: String,
    /// Terminal state
    pub state: TerminalState,
    /// Terminal mode
    pub mode: TerminalMode,
    /// Terminal config
    pub config: TerminalConfig,
    /// Terminal title
    pub title: String,
    /// Terminal working directory
    pub working_directory: String,
    /// Terminal command
    pub command: String,
    /// Terminal process ID
    pub process_id: Option<u32>,
    /// Terminal created timestamp
    pub created_at: u64,
    /// Terminal last activity timestamp
    pub last_activity: u64,
    /// Terminal window width
    pub width: u32,
    /// Terminal window height
    pub height: u32,
    /// Terminal scroll position
    pub scroll_position: usize,
    /// Terminal scrollback size
    pub scrollback_size: usize,
    /// Terminal visible
    pub visible: bool,
}

impl Terminal {
    /// Create new terminal
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: TerminalState::Idle,
            mode: TerminalMode::Normal,
            config: TerminalConfig::default(),
            title: "Terminal".into(),
            working_directory: "/".into(),
            command: String::new(),
            process_id: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            width: 800,
            height: 600,
            scroll_position: 0,
            scrollback_size: 0,
            visible: true,
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.update_activity();
    }
    
    /// Set working directory
    pub fn set_working_directory(&mut self, directory: String) {
        self.working_directory = directory;
        self.update_activity();
    }
    
    /// Set command
    pub fn set_command(&mut self, command: String) {
        self.command = command;
        self.update_activity();
    }
    
    /// Set process ID
    pub fn set_process_id(&mut self, process_id: u32) {
        self.process_id = Some(process_id);
        self.update_activity();
    }
    
    /// Set config
    pub fn set_config(&mut self, config: TerminalConfig) {
        self.config = config;
        self.update_activity();
    }
    
    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.update_activity();
    }
    
    /// Set scroll position
    pub fn set_scroll_position(&mut self, position: usize) {
        self.scroll_position = position;
        self.update_activity();
    }
    
    /// Set scrollback size
    pub fn set_scrollback_size(&mut self, size: usize) {
        self.scrollback_size = size;
        self.update_activity();
    }
    
    /// Set visible
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.update_activity();
    }
    
    /// Start terminal
    pub fn start(&mut self) -> bool {
        if self.state == TerminalState::Idle {
            self.state = TerminalState::Running;
            self.update_activity();
            return true;
        }
        false
    }
    
    /// Pause terminal
    pub fn pause(&mut self) -> bool {
        if self.state == TerminalState::Running {
            self.state = TerminalState::Paused;
            self.update_activity();
            return true;
        }
        false
    }
    
    /// Resume terminal
    pub fn resume(&mut self) -> bool {
        if self.state == TerminalState::Paused {
            self.state = TerminalState::Running;
            self.update_activity();
            return true;
        }
        false
    }
    
    /// Stop terminal
    pub fn stop(&mut self) -> bool {
        if self.state == TerminalState::Running || self.state == TerminalState::Paused {
            self.state = TerminalState::Exited;
            self.update_activity();
            return true;
        }
        false
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = TerminalState::Error;
        self.update_activity();
    }
    
    /// Update activity
    fn update_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get terminal age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since last activity in seconds
    pub fn get_time_since_activity(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.last_activity
    }
    
    /// Check if terminal is running
    pub fn is_running(&self) -> bool {
        self.state == TerminalState::Running
    }
    
    /// Check if terminal is paused
    pub fn is_paused(&self) -> bool {
        self.state == TerminalState::Paused
    }
    
    /// Check if terminal is exited
    pub fn is_exited(&self) -> bool {
        self.state == TerminalState::Exited
    }
    
    /// Check if terminal has error
    pub fn has_error(&self) -> bool {
        self.state == TerminalState::Error
    }
    
    /// Check if terminal is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Check if terminal is idle
    pub fn is_idle(&self) -> bool {
        self.state == TerminalState::Idle
    }
    
    /// Get terminal info
    pub fn get_info(&self) -> TerminalInfo {
        TerminalInfo {
            id: self.id.clone(),
            title: self.title.clone(),
            state: self.state.clone(),
            mode: self.mode.clone(),
            working_directory: self.working_directory.clone(),
            command: self.command.clone(),
            process_id: self.process_id,
            width: self.width,
            height: self.height,
            scrollback_size: self.scrollback_size,
            visible: self.visible,
        }
    }
}

/// Terminal info
#[derive(Debug, Clone)]
pub struct TerminalInfo {
    /// Terminal ID
    pub id: String,
    /// Terminal title
    pub title: String,
    /// Terminal state
    pub state: TerminalState,
    /// Terminal mode
    pub mode: TerminalMode,
    /// Working directory
    pub working_directory: String,
    /// Command
    pub command: String,
    /// Process ID
    pub process_id: Option<u32>,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Scrollback size
    pub scrollback_size: usize,
    /// Visible
    pub visible: bool,
}
