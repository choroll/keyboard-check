use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
        // Exit shortcuts should only trigger on press/repeat events.
        if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            match key.code {
                KeyCode::Esc => {
                    self.running = false;
                }
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                    self.running = false;
                }
                _ => {}
            }
        }

        // Keys light up while pressed and clear on release.
        match key.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                self.pressed_keys.insert(key.code);
            }
            KeyEventKind::Release => {
                self.pressed_keys.remove(&key.code);
            }
        }
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
        app.handle_key_event(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));
        assert!(!app.is_pressed(KeyCode::Char('a')));
    }

    #[test]
    fn test_enter_not_pressed_by_default() {
        let app = App::new();
        assert!(!app.is_pressed(KeyCode::Enter));
    }

    #[test]
    fn test_release_event_does_not_light_key() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new_with_kind(
            KeyCode::Enter,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));
        assert!(!app.is_pressed(KeyCode::Enter));
    }

    #[test]
    fn test_enter_release_clears_highlight() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::from(KeyCode::Enter));
        assert!(app.is_pressed(KeyCode::Enter));

        app.handle_key_event(KeyEvent::new_with_kind(
            KeyCode::Enter,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));
        assert!(!app.is_pressed(KeyCode::Enter));
    }

    #[test]
    fn test_esc_exits() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::from(KeyCode::Esc));
        assert!(!app.running);
    }
}
