//! Session management for alacritty.

/// Session type
#[derive(Debug, Clone, PartialEq)]
pub enum SessionType {
    /// Shell session
    Shell,
    /// SSH session
    SSH,
    /// Local session
    Local,
    /// Remote session
    Remote,
    /// Custom session
    Custom,
}

/// Session status
#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    /// Active session
    Active,
    /// Inactive session
    Inactive,
    /// Exited session
    Exited,
    /// Error session
    Error,
}

/// Session config
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Session command
    pub command: String,
    /// Session arguments
    pub arguments: Vec<String>,
    /// Session environment variables
    pub environment: HashMap<String, String>,
    /// Session working directory
    pub working_directory: String,
    /// Session terminal type
    pub terminal_type: String,
    /// Session shell
    pub shell: String,
    /// Session auto-restart
    pub auto_restart: bool,
    /// Session restart delay in seconds
    pub restart_delay: u64,
}

use std::collections::HashMap;

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            arguments: vec![],
            environment: HashMap::new(),
            working_directory: "/".into(),
            terminal_type: "xterm-256color".into(),
            shell: "/bin/sh".into(),
            auto_restart: false,
            restart_delay: 5,
        }
    }
}

impl SessionConfig {
    /// Create new session config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set command
    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }
    
    /// Add argument
    pub fn add_argument(&mut self, argument: String) {
        self.arguments.push(argument);
    }
    
    /// Set environment variable
    pub fn set_environment(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
    }
    
    /// Set working directory
    pub fn set_working_directory(&mut self, directory: String) {
        self.working_directory = directory;
    }
    
    /// Set terminal type
    pub fn set_terminal_type(&mut self, terminal_type: String) {
        self.terminal_type = terminal_type;
    }
    
    /// Set shell
    pub fn set_shell(&mut self, shell: String) {
        self.shell = shell;
    }
    
    /// Enable auto-restart
    pub fn enable_auto_restart(&mut self) {
        self.auto_restart = true;
    }
    
    /// Disable auto-restart
    pub fn disable_auto_restart(&mut self) {
        self.auto_restart = false;
    }
    
    /// Set restart delay
    pub fn set_restart_delay(&mut self, delay: u64) {
        self.restart_delay = delay;
    }
}

/// Session
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub id: String,
    /// Session type
    pub session_type: SessionType,
    /// Session status
    pub status: SessionStatus,
    /// Session config
    pub config: SessionConfig,
    /// Session title
    pub title: String,
    /// Session process ID
    pub process_id: Option<u32>,
    /// Session created timestamp
    pub created_at: u64,
    /// Session last activity timestamp
    pub last_activity: u64,
    /// Session exit code
    pub exit_code: Option<i32>,
    /// Session restart count
    pub restart_count: u32,
    /// Session visible
    pub visible: bool,
}

impl Session {
    /// Create new session
    pub fn new(id: String, session_type: SessionType) -> Self {
        Self {
            id,
            session_type,
            status: SessionStatus::Inactive,
            config: SessionConfig::default(),
            title: "Session".into(),
            process_id: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            exit_code: None,
            restart_count: 0,
            visible: true,
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.update_activity();
    }
    
    /// Set config
    pub fn set_config(&mut self, config: SessionConfig) {
        self.config = config;
        self.update_activity();
    }
    
    /// Set process ID
    pub fn set_process_id(&mut self, process_id: u32) {
        self.process_id = Some(process_id);
        self.update_activity();
    }
    
    /// Set exit code
    pub fn set_exit_code(&mut self, exit_code: i32) {
        self.exit_code = Some(exit_code);
        self.update_activity();
    }
    
    /// Increment restart count
    pub fn increment_restart_count(&mut self) {
        self.restart_count += 1;
        self.update_activity();
    }
    
    /// Set visible
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.update_activity();
    }
    
    /// Start session
    pub fn start(&mut self) -> bool {
        if self.status == SessionStatus::Inactive {
            self.status = SessionStatus::Active;
            self.update_activity();
            return true;
        }
        false
    }
    
    /// Stop session
    pub fn stop(&mut self) -> bool {
        if self.status == SessionStatus::Active {
            self.status = SessionStatus::Exited;
            self.update_activity();
            return true;
        }
        false
    }
    
    /// Set error status
    pub fn set_error(&mut self) {
        self.status = SessionStatus::Error;
        self.update_activity();
    }
    
    /// Update activity
    fn update_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get session age in seconds
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
    
    /// Check if session is active
    pub fn is_active(&self) -> bool {
        self.status == SessionStatus::Active
    }
    
    /// Check if session is exited
    pub fn is_exited(&self) -> bool {
        self.status == SessionStatus::Exited
    }
    
    /// Check if session has error
    pub fn has_error(&self) -> bool {
        self.status == SessionStatus::Error
    }
    
    /// Check if session is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Check if session is inactive
    pub fn is_inactive(&self) -> bool {
        self.status == SessionStatus::Inactive
    }
    
    /// Get session info
    pub fn get_info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.clone(),
            title: self.title.clone(),
            session_type: self.session_type.clone(),
            status: self.status.clone(),
            process_id: self.process_id,
            exit_code: self.exit_code,
            restart_count: self.restart_count,
            visible: self.visible,
        }
    }
}

/// Session info
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Session ID
    pub id: String,
    /// Session title
    pub title: String,
    /// Session type
    pub session_type: SessionType,
    /// Session status
    pub status: SessionStatus,
    /// Process ID
    pub process_id: Option<u32>,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Restart count
    pub restart_count: u32,
    /// Visible
    pub visible: bool,
}
