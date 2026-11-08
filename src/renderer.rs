//! Renderer for alacritty.

/// Render backend
#[derive(Debug, Clone, PartialEq)]
pub enum RenderBackend {
    /// GPU backend
    GPU,
    /// CPU backend
    CPU,
    /// Auto backend
    Auto,
}

/// Render quality
#[derive(Debug, Clone, PartialEq)]
pub enum RenderQuality {
    /// Low quality
    Low,
    /// Medium quality
    Medium,
    /// High quality
    High,
    /// Ultra quality
    Ultra,
}

/// Render config
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Render backend
    pub backend: RenderBackend,
    /// Render quality
    pub quality: RenderQuality,
    /// Render width
    pub width: u32,
    /// Render height
    pub height: u32,
    /// Render scale factor
    pub scale_factor: f64,
    /// Render vsync
    pub vsync: bool,
    /// Render frame rate limit
    pub frame_rate_limit: u32,
    /// Render text rendering
    pub text_rendering: TextRendering,
}

/// Text rendering
#[derive(Debug, Clone, PartialEq)]
pub enum TextRendering {
    /// Default text rendering
    Default,
    /// Subpixel text rendering
    Subpixel,
    /// Grayscale text rendering
    Grayscale,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            backend: RenderBackend::Auto,
            quality: RenderQuality::High,
            width: 800,
            height: 600,
            scale_factor: 1.0,
            vsync: true,
            frame_rate_limit: 60,
            text_rendering: TextRendering::Subpixel,
        }
    }
}

/// Renderer
#[derive(Debug, Clone)]
pub struct Renderer {
    /// Renderer backend
    pub backend: RenderBackend,
    /// Renderer config
    pub config: RenderConfig,
    /// Renderer running
    pub running: bool,
    /// Renderer frame count
    pub frame_count: u64,
    /// Renderer last frame timestamp
    pub last_frame: u64,
    /// Renderer created timestamp
    pub created_at: u64,
}

impl Renderer {
    /// Create new renderer
    pub fn new(backend: RenderBackend) -> Self {
        Self {
            backend,
            config: RenderConfig::default(),
            running: false,
            frame_count: 0,
            last_frame: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: RenderConfig) {
        self.config = config;
    }
    
    /// Start renderer
    pub fn start(&mut self) -> bool {
        if !self.running {
            self.running = true;
            self.last_frame = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return true;
        }
        false
    }
    
    /// Stop renderer
    pub fn stop(&mut self) -> bool {
        if self.running {
            self.running = false;
            return true;
        }
        false
    }
    
    /// Render frame
    pub fn render_frame(&mut self) -> bool {
        if self.running {
            self.frame_count += 1;
            self.last_frame = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return true;
        }
        false
    }
    
    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
    }
    
    /// Set scale factor
    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.config.scale_factor = scale_factor;
    }
    
    /// Set quality
    pub fn set_quality(&mut self, quality: RenderQuality) {
        self.config.quality = quality;
    }
    
    /// Enable vsync
    pub fn enable_vsync(&mut self) {
        self.config.vsync = true;
    }
    
    /// Disable vsync
    pub fn disable_vsync(&mut self) {
        self.config.vsync = false;
    }
    
    /// Set frame rate limit
    pub fn set_frame_rate_limit(&mut self, limit: u32) {
        self.config.frame_rate_limit = limit;
    }
    
    /// Set text rendering
    pub fn set_text_rendering(&mut self, text_rendering: TextRendering) {
        self.config.text_rendering = text_rendering;
    }
    
    /// Get renderer age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get frame rate
    pub fn get_frame_rate(&self) -> f64 {
        if self.frame_count == 0 || self.last_frame == 0 {
            return 0.0;
        }
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let elapsed = now - self.last_frame;
        if elapsed == 0 {
            return 0.0;
        }
        
        self.frame_count as f64 / elapsed as f64
    }
    
    /// Get frame count
    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }
    
    /// Check if renderer is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}
