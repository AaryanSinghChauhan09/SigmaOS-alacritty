//! Input handling for alacritty.

/// Input event
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Key press event
    KeyPress { key: String, modifiers: Vec<String> },
    /// Key release event
    KeyRelease { key: String, modifiers: Vec<String> },
    /// Mouse click event
    MouseClick { x: i32, y: i32, button: MouseButton },
    /// Mouse move event
    MouseMove { x: i32, y: i32 },
    /// Mouse scroll event
    MouseScroll { x: i32, y: i32, delta: i32 },
    /// Paste event
    Paste { text: String },
    /// Focus event
    Focus { focused: bool },
}

/// Mouse button
#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    /// Left button
    Left,
    /// Right button
    Right,
    /// Middle button
    Middle,
}

/// Key action
#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    /// Send input
    SendInput,
    /// Copy text
    Copy,
    /// Paste text
    Paste,
    /// Clear screen
    Clear,
    /// Exit terminal
    Exit,
    /// New tab
    NewTab,
    /// Close tab
    CloseTab,
    /// Next tab
    NextTab,
    /// Previous tab
    PreviousTab,
    /// Custom action
    Custom(String),
}

/// Key binding
#[derive(Debug, Clone)]
pub struct KeyBinding {
    /// Key binding key
    pub key: String,
    /// Key binding modifiers
    pub modifiers: Vec<String>,
    /// Key binding action
    pub action: KeyAction,
    /// Key binding description
    pub description: String,
}

impl KeyBinding {
    /// Create new key binding
    pub fn new(key: String, modifiers: Vec<String>, action: KeyAction) -> Self {
        Self {
            key,
            modifiers,
            action,
            description: String::new(),
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    
    /// Check if binding matches event
    pub fn matches(&self, event: &InputEvent) -> bool {
        match event {
            InputEvent::KeyPress { key, modifiers } => {
                self.key == *key && self.modifiers == *modifiers
            }
            _ => false,
        }
    }
}

/// Input handler
#[derive(Debug, Clone)]
pub struct InputHandler {
    /// Key bindings
    pub key_bindings: Vec<KeyBinding>,
    /// Input handler running
    pub running: bool,
    /// Input handler created timestamp
    pub created_at: u64,
    /// Input handler event count
    pub event_count: u64,
}

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            key_bindings: vec![],
            running: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_count: 0,
        }
    }
}

impl InputHandler {
    /// Create new input handler
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add key binding
    pub fn add_key_binding(&mut self, binding: KeyBinding) {
        self.key_bindings.push(binding);
    }
    
    /// Remove key binding
    pub fn remove_key_binding(&mut self, key: &str, modifiers: &[String]) -> bool {
        if let Some(pos) = self.key_bindings.iter().position(|b| b.key == key && b.modifiers == modifiers) {
            self.key_bindings.remove(pos);
            return true;
        }
        false
    }
    
    /// Get key bindings
    pub fn get_key_bindings(&self) -> &[KeyBinding] {
        &self.key_bindings
    }
    
    /// Start input handler
    pub fn start(&mut self) -> bool {
        if !self.running {
            self.running = true;
            return true;
        }
        false
    }
    
    /// Stop input handler
    pub fn stop(&mut self) -> bool {
        if self.running {
            self.running = false;
            return true;
        }
        false
    }
    
    /// Handle input event
    pub fn handle_event(&mut self, event: InputEvent) -> Option<KeyAction> {
        self.event_count += 1;
        
        for binding in &self.key_bindings {
            if binding.matches(&event) {
                return Some(binding.action.clone());
            }
        }
        
        None
    }
    
    /// Get input handler age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get event count
    pub fn get_event_count(&self) -> u64 {
        self.event_count
    }
    
    /// Check if input handler is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}
