//! alacritty model — GPU-accelerated terminal.

#[derive(Debug, Clone)]
pub struct Config {
    pub font: String,
    pub font_size: u32,
    pub theme: String,
    pub opacity: f64,
    pub scrollback: usize,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub command: String,
    pub config: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub configs: Vec<Config>,
    pub sessions: Vec<Session>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { configs: vec![], sessions: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_config(&mut self, config: Config) { self.configs.push(config); }

    pub fn new_session(&mut self, command: &str, config: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.sessions.push(Session { id: id.clone(), command: command.into(), config: config.into() });
        id
    }

    pub fn close_session(&mut self, id: &str) -> bool {
        if let Some(pos) = self.sessions.iter().position(|s| s.id == id) {
            self.sessions.remove(pos);
            true
        } else { false }
    }

    pub fn list_sessions(&self) -> &[Session] { &self.sessions }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_session() {
        let mut a = Model::new();
        a.add_config(Config { font: "Fira Code".into(), font_size: 14, theme: "dark".into(), opacity: 0.9, scrollback: 10000 });
        let id = a.new_session("bash", "default");
        assert!(a.close_session(&id));
    }
}
