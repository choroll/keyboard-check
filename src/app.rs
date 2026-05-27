use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PressedKey {
    code: KeyCode,
    is_keypad: bool,
}

impl PressedKey {
    fn from_event(key: KeyEvent) -> Self {
        Self {
            code: key.code,
            is_keypad: key.state.contains(KeyEventState::KEYPAD),
        }
    }
}

pub struct App {
    pressed_keys: HashSet<PressedKey>,
    pressed_modifiers: KeyModifiers,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            pressed_modifiers: KeyModifiers::NONE,
            running: true,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        self.pressed_modifiers = key.modifiers;

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
        let pressed_key = PressedKey::from_event(key);
        match key.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                self.pressed_keys.insert(pressed_key);
            }
            KeyEventKind::Release => {
                if !self.pressed_keys.remove(&pressed_key) {
                    self.pressed_keys.retain(|pressed| pressed.code != key.code);
                }
            }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.iter().any(|pressed| pressed.code == key)
    }

    pub fn is_pressed_non_keypad(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&PressedKey {
            code: key,
            is_keypad: false,
        })
    }

    pub fn is_pressed_on_keypad(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&PressedKey {
            code: key,
            is_keypad: true,
        })
    }

    pub fn is_modifier_pressed(&self, modifier: ModifierKeyCode) -> bool {
        self.is_pressed(KeyCode::Modifier(modifier))
            || self.pressed_modifiers.contains(match modifier {
                ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => KeyModifiers::SHIFT,
                ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => {
                    KeyModifiers::CONTROL
                }
                ModifierKeyCode::LeftAlt
                | ModifierKeyCode::RightAlt
                | ModifierKeyCode::IsoLevel3Shift
                | ModifierKeyCode::IsoLevel5Shift => KeyModifiers::ALT,
                ModifierKeyCode::LeftSuper | ModifierKeyCode::RightSuper => KeyModifiers::SUPER,
                ModifierKeyCode::LeftHyper | ModifierKeyCode::RightHyper => KeyModifiers::HYPER,
                ModifierKeyCode::LeftMeta | ModifierKeyCode::RightMeta => KeyModifiers::META,
            })
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
    fn test_keypad_state_is_tracked_separately() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new_with_kind_and_state(
            KeyCode::Char('7'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
            KeyEventState::KEYPAD,
        ));

        assert!(app.is_pressed(KeyCode::Char('7')));
        assert!(app.is_pressed_on_keypad(KeyCode::Char('7')));
        assert!(!app.is_pressed_non_keypad(KeyCode::Char('7')));
    }

    #[test]
    fn test_release_without_keypad_state_clears_keypad_press() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new_with_kind_and_state(
            KeyCode::Char('7'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
            KeyEventState::KEYPAD,
        ));
        app.handle_key_event(KeyEvent::new_with_kind(
            KeyCode::Char('7'),
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));

        assert!(!app.is_pressed(KeyCode::Char('7')));
    }

    #[test]
    fn test_modifier_state_is_tracked() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT));

        assert!(app.is_modifier_pressed(ModifierKeyCode::LeftShift));
        assert!(app.is_modifier_pressed(ModifierKeyCode::RightShift));
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
