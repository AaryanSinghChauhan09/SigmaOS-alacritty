//! SigmaOS-alacritty — comprehensive GPU-accelerated terminal emulator component.
//!
//! Provides terminal emulator functionality with GPU acceleration,
//! configuration management, session handling, and window management.

pub mod terminal;
pub mod config;
pub mod session;
pub mod renderer;
pub mod input;
pub mod manager;
pub mod options;

pub use terminal::{Terminal, TerminalMode, TerminalState, TerminalConfig};
pub use config::{Theme, FontConfig, ColorScheme};
pub use session::{Session, SessionType, SessionStatus, SessionConfig};
pub use renderer::{Renderer, RenderConfig, RenderBackend, RenderQuality};
pub use input::{InputHandler, InputEvent, KeyBinding, KeyAction};
pub use manager::{TerminalManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_terminal_mode() {
        let modes = vec![
            TerminalMode::Normal,
            TerminalMode::Alternate,
            TerminalMode::Scrollback,
            TerminalMode::Selection,
            TerminalMode::Search,
        ];
        assert_eq!(modes.len(), 5);
    }
    
    #[test]
    fn test_terminal_state() {
        let states = vec![
            TerminalState::Idle,
            TerminalState::Running,
            TerminalState::Paused,
            TerminalState::Exited,
            TerminalState::Error,
        ];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_session_type() {
        let types = vec![
            SessionType::Shell,
            SessionType::SSH,
            SessionType::Local,
            SessionType::Remote,
            SessionType::Custom,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_session_status() {
        let statuses = vec![
            SessionStatus::Active,
            SessionStatus::Inactive,
            SessionStatus::Exited,
            SessionStatus::Error,
        ];
        assert_eq!(statuses.len(), 4);
    }
    
    #[test]
    fn test_render_backend() {
        let backends = vec![
            RenderBackend::GPU,
            RenderBackend::CPU,
            RenderBackend::Auto,
        ];
        assert_eq!(backends.len(), 3);
    }
    
    #[test]
    fn test_render_quality() {
        let qualities = vec![
            RenderQuality::Low,
            RenderQuality::Medium,
            RenderQuality::High,
            RenderQuality::Ultra,
        ];
        assert_eq!(qualities.len(), 4);
    }
    
    #[test]
    fn test_input_event() {
        let event = InputEvent::KeyPress { key: "Enter".into(), modifiers: vec![] };
        assert!(matches!(event, InputEvent::KeyPress { .. }));
    }
    
    #[test]
    fn test_key_action() {
        let actions = vec![
            KeyAction::SendInput,
            KeyAction::Copy,
            KeyAction::Paste,
            KeyAction::Clear,
            KeyAction::Exit,
        ];
        assert_eq!(actions.len(), 5);
    }
    
    #[test]
    fn test_terminal_config_creation() {
        let mut config = TerminalConfig::default();
        config.font_size = 14;
        config.theme = "dark".into();
        config.opacity = 0.95;
        
        assert_eq!(config.font_size, 14);
        assert_eq!(config.theme, "dark");
        assert_eq!(config.opacity, 0.95);
    }
    
    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("dark".into());
        assert_eq!(theme.name, "dark");
        assert!(theme.colors.is_empty());
    }
    
    #[test]
    fn test_font_config_creation() {
        let font = FontConfig::new("Monospace".into(), 12);
        assert_eq!(font.family, "Monospace");
        assert_eq!(font.size, 12);
    }
    
    #[test]
    fn test_color_scheme_creation() {
        let scheme = ColorScheme::new();
        assert!(scheme.colors.is_empty());
    }
    
    #[test]
    fn test_terminal_creation() {
        let terminal = Terminal::new("term-1".into());
        assert_eq!(terminal.id, "term-1");
        assert_eq!(terminal.state, TerminalState::Idle);
    }
    
    #[test]
    fn test_session_creation() {
        let session = Session::new("sess-1".into(), SessionType::Shell);
        assert_eq!(session.id, "sess-1");
        assert_eq!(session.session_type, SessionType::Shell);
    }
    
    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(RenderBackend::GPU);
        assert_eq!(renderer.backend, RenderBackend::GPU);
    }
    
    #[test]
    fn test_input_handler_creation() {
        let handler = InputHandler::new();
        assert!(handler.key_bindings.is_empty());
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = TerminalManager::new();
        assert!(manager.list_terminals().is_empty());
        assert!(manager.list_sessions().is_empty());
    }
    
    #[test]
    fn test_create_terminal() {
        let mut manager = TerminalManager::new();
        let terminal_id = manager.create_terminal();
        
        assert!(manager.get_terminal(&terminal_id).is_some());
        assert_eq!(manager.list_terminals().len(), 1);
    }
    
    #[test]
    fn test_create_session() {
        let mut manager = TerminalManager::new();
        let session_id = manager.create_session(SessionType::Shell);
        
        assert!(manager.get_session(&session_id).is_some());
        assert_eq!(manager.list_sessions().len(), 1);
    }
    
    #[test]
    fn test_close_session() {
        let mut manager = TerminalManager::new();
        let session_id = manager.create_session(SessionType::Shell);
        
        // Start session first
        if let Some(session) = manager.get_session_mut(&session_id) {
            session.start();
        }
        
        assert!(manager.close_session(&session_id));
        assert!(manager.get_session(&session_id).is_some()); // Session still exists but is exited
        assert_eq!(manager.get_session(&session_id).unwrap().status, SessionStatus::Exited);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = TerminalManager::new();
        
        manager.create_terminal();
        manager.create_session(SessionType::Shell);
        
        let export = manager.export_data();
        
        let mut manager2 = TerminalManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_terminals().len(), 1);
        assert_eq!(manager2.list_sessions().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = TerminalManager::new();
        
        manager.create_terminal();
        manager.create_terminal();
        manager.create_session(SessionType::Shell);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_terminals, 2);
        assert_eq!(stats.total_sessions, 1);
    }
}
