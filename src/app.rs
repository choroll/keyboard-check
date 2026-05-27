use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashSet;

pub struct App {
    pub pressed_keys: HashSet<KeyCode>,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            running: true,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.running = false;
            }
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                self.running = false;
            }
            _ => {
                self.pressed_keys.insert(key.code);
            }
        }
    }

    #[allow(dead_code)]
    pub fn release_key(&mut self, key: KeyCode) {
        self.pressed_keys.remove(&key);
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_press() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::from(KeyCode::Char('a')));
        assert!(app.is_pressed(KeyCode::Char('a')));
    }

    #[test]
    fn test_key_release() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::from(KeyCode::Char('a')));
        app.release_key(KeyCode::Char('a'));
        assert!(!app.is_pressed(KeyCode::Char('a')));
    }

    #[test]
    fn test_esc_exits() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::from(KeyCode::Esc));
        assert!(!app.running);
    }
}
