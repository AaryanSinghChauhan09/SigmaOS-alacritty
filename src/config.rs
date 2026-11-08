//! Configuration for alacritty.

use std::collections::HashMap;

/// Theme
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme colors
    pub colors: HashMap<String, String>,
    /// Theme background color
    pub background_color: String,
    /// Theme foreground color
    pub foreground_color: String,
    /// Theme cursor color
    pub cursor_color: String,
    /// Theme selection color
    pub selection_color: String,
}

impl Theme {
    /// Create new theme
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: HashMap::new(),
            background_color: "#000000".into(),
            foreground_color: "#ffffff".into(),
            cursor_color: "#ffffff".into(),
            selection_color: "#333333".into(),
        }
    }
    
    /// Set color
    pub fn set_color(&mut self, name: String, color: String) {
        self.colors.insert(name, color);
    }
    
    /// Get color
    pub fn get_color(&self, name: &str) -> Option<&String> {
        self.colors.get(name)
    }
    
    /// Set background color
    pub fn set_background_color(&mut self, color: String) {
        self.background_color = color;
    }
    
    /// Set foreground color
    pub fn set_foreground_color(&mut self, color: String) {
        self.foreground_color = color;
    }
    
    /// Set cursor color
    pub fn set_cursor_color(&mut self, color: String) {
        self.cursor_color = color;
    }
    
    /// Set selection color
    pub fn set_selection_color(&mut self, color: String) {
        self.selection_color = color;
    }
}

/// Font config
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Font family
    pub family: String,
    /// Font size
    pub size: u32,
    /// Font weight
    pub weight: FontWeight,
    /// Font style
    pub style: FontStyle,
    /// Font hinting
    pub hinting: bool,
    /// Font subpixel rendering
    pub subpixel_rendering: bool,
}

/// Font weight
#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    /// Thin weight
    Thin,
    /// Light weight
    Light,
    /// Normal weight
    Normal,
    /// Bold weight
    Bold,
    /// Black weight
    Black,
}

/// Font style
#[derive(Debug, Clone, PartialEq)]
pub enum FontStyle {
    /// Normal style
    Normal,
    /// Italic style
    Italic,
    /// Oblique style
    Oblique,
}

impl FontConfig {
    /// Create new font config
    pub fn new(family: String, size: u32) -> Self {
        Self {
            family,
            size,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            hinting: true,
            subpixel_rendering: true,
        }
    }
    
    /// Set weight
    pub fn set_weight(&mut self, weight: FontWeight) {
        self.weight = weight;
    }
    
    /// Set style
    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }
    
    /// Enable hinting
    pub fn enable_hinting(&mut self) {
        self.hinting = true;
    }
    
    /// Disable hinting
    pub fn disable_hinting(&mut self) {
        self.hinting = false;
    }
    
    /// Enable subpixel rendering
    pub fn enable_subpixel_rendering(&mut self) {
        self.subpixel_rendering = true;
    }
    
    /// Disable subpixel rendering
    pub fn disable_subpixel_rendering(&mut self) {
        self.subpixel_rendering = false;
    }
}

/// Color scheme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// Color scheme name
    pub name: String,
    /// Color scheme colors
    pub colors: HashMap<String, String>,
    /// Color scheme background color
    pub background_color: String,
    /// Color scheme foreground color
    pub foreground_color: String,
    /// Color scheme cursor color
    pub cursor_color: String,
    /// Color scheme selection color
    pub selection_color: String,
    /// Color scheme bright colors
    pub bright_colors: HashMap<String, String>,
    /// Color scheme dim colors
    pub dim_colors: HashMap<String, String>,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            name: "default".into(),
            colors: HashMap::new(),
            background_color: "#000000".into(),
            foreground_color: "#ffffff".into(),
            cursor_color: "#ffffff".into(),
            selection_color: "#333333".into(),
            bright_colors: HashMap::new(),
            dim_colors: HashMap::new(),
        }
    }
}

impl ColorScheme {
    /// Create new color scheme
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Set color
    pub fn set_color(&mut self, name: String, color: String) {
        self.colors.insert(name, color);
    }
    
    /// Get color
    pub fn get_color(&self, name: &str) -> Option<&String> {
        self.colors.get(name)
    }
    
    /// Set background color
    pub fn set_background_color(&mut self, color: String) {
        self.background_color = color;
    }
    
    /// Set foreground color
    pub fn set_foreground_color(&mut self, color: String) {
        self.foreground_color = color;
    }
    
    /// Set cursor color
    pub fn set_cursor_color(&mut self, color: String) {
        self.cursor_color = color;
    }
    
    /// Set selection color
    pub fn set_selection_color(&mut self, color: String) {
        self.selection_color = color;
    }
    
    /// Set bright color
    pub fn set_bright_color(&mut self, name: String, color: String) {
        self.bright_colors.insert(name, color);
    }
    
    /// Get bright color
    pub fn get_bright_color(&self, name: &str) -> Option<&String> {
        self.bright_colors.get(name)
    }
    
    /// Set dim color
    pub fn set_dim_color(&mut self, name: String, color: String) {
        self.dim_colors.insert(name, color);
    }
    
    /// Get dim color
    pub fn get_dim_color(&self, name: &str) -> Option<&String> {
        self.dim_colors.get(name)
    }
}

/// Terminal config (alias for compatibility)
pub type Config = crate::terminal::TerminalConfig;
