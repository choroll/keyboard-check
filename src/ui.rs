use crossterm::event::{KeyCode, ModifierKeyCode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

const KEY_HEIGHT: u16 = 3;
const MAIN_NAV_GAP: u16 = 74;
const MAIN_NUMPAD_GAP: u16 = 93;

#[derive(Clone, Copy)]
enum KeyMatcher {
    Code(KeyCode),
    NonKeypadCode(KeyCode),
    Chars(&'static str),
    KeypadChars(&'static str),
    KeypadCodes(&'static [KeyCode]),
    Modifier(ModifierKeyCode),
}

#[derive(Clone, Copy)]
struct KeySpec {
    label: &'static str,
    width: u16,
    matcher: KeyMatcher,
}

#[derive(Clone, Copy)]
enum KeyUnit {
    Key(KeySpec),
    Gap(u16),
}

#[derive(Clone, Copy)]
struct PositionedKey {
    row: u16,
    col: u16,
    height: u16,
    key: KeySpec,
}

const fn spec(label: &'static str, width: u16, matcher: KeyMatcher) -> KeySpec {
    KeySpec {
        label,
        width,
        matcher,
    }
}

const fn key(label: &'static str, width: u16, matcher: KeyMatcher) -> KeyUnit {
    KeyUnit::Key(spec(label, width, matcher))
}

const fn gap(width: u16) -> KeyUnit {
    KeyUnit::Gap(width)
}

// ========== 按键分类与配色系统 ==========

#[derive(Clone, Copy, Debug)]
enum KeyCategory {
    Function,   // F1-F12, Esc, Print, etc.
    Alpha,      // A-Z
    Number,     // 0-9
    Modifier,   // Shift, Ctrl, Alt, Win
    Navigation, // Insert, Delete, Home, End, PgUp, PgDn
    Arrow,      // Up/Down/Left/Right
    Numpad,     // NumLock, keypad numbers & operators
    Special,    // Space, Enter, Tab, Backspace, Caps, symbols
}

impl KeyCategory {
    /// 返回 (背景色, 文字色, 边框色)
    fn colors(self, pressed: bool) -> (Color, Color, Color) {
        if pressed {
            match self {
                Self::Function => {
                    (Color::Rgb(100, 200, 255), Color::Black, Color::Rgb(50, 150, 255))
                }
                Self::Alpha => {
                    (Color::Rgb(100, 255, 100), Color::Black, Color::Rgb(50, 200, 50))
                }
                Self::Number => {
                    (Color::Rgb(255, 255, 100), Color::Black, Color::Rgb(200, 200, 50))
                }
                Self::Modifier => {
                    (Color::Rgb(255, 120, 120), Color::Black, Color::Rgb(220, 80, 80))
                }
                Self::Navigation => {
                    (Color::Rgb(255, 120, 255), Color::Black, Color::Rgb(220, 80, 220))
                }
                Self::Arrow => {
                    (Color::Rgb(120, 150, 255), Color::Black, Color::Rgb(80, 100, 220))
                }
                Self::Numpad => {
                    (Color::Rgb(255, 200, 80), Color::Black, Color::Rgb(220, 160, 40))
                }
                Self::Special => {
                    (Color::Rgb(220, 220, 220), Color::Black, Color::Rgb(180, 180, 180))
                }
            }
        } else {
            // 未按下：统一的暗色主题
            (
                Color::Rgb(42, 42, 42),
                Color::Rgb(170, 170, 170),
                Color::Rgb(65, 65, 65),
            )
        }
    }
}

/// 根据 KeyMatcher 推断按键类别
fn infer_category(matcher: KeyMatcher) -> KeyCategory {
    use KeyCode::*;
    match matcher {
        // 功能键
        KeyMatcher::Code(Esc | F(_) | PrintScreen | ScrollLock | Pause) => KeyCategory::Function,
        // NumLock 属于数字键盘区
        KeyMatcher::Code(NumLock) => KeyCategory::Numpad,
        // 修饰键
        KeyMatcher::Modifier(_) => KeyCategory::Modifier,
        // 方向键（独立的方向键，非数字键盘上的）
        KeyMatcher::Code(Up | Down | Left | Right) => KeyCategory::Arrow,
        // 导航键
        KeyMatcher::NonKeypadCode(Insert | Delete | Home | End | PageUp | PageDown) => {
            KeyCategory::Navigation
        }
        // 数字键盘
        KeyMatcher::KeypadChars(_) | KeyMatcher::KeypadCodes(_) => KeyCategory::Numpad,
        // 字符键：根据首字符判断是字母、数字还是符号
        KeyMatcher::Chars(chars) => {
            let first = chars.as_bytes()[0];
            match first {
                b'a'..=b'z' | b'A'..=b'Z' => KeyCategory::Alpha,
                b'0'..=b'9' => KeyCategory::Number,
                _ => KeyCategory::Special,
            }
        }
        // 其他所有键归为特殊键
        _ => KeyCategory::Special,
    }
}

// ========== 键盘布局定义 ==========

const FUNCTION_ROW: &[KeyUnit] = &[
    key("Esc", 5, KeyMatcher::Code(KeyCode::Esc)),
    gap(2),
    key("F1", 4, KeyMatcher::Code(KeyCode::F(1))),
    key("F2", 4, KeyMatcher::Code(KeyCode::F(2))),
    key("F3", 4, KeyMatcher::Code(KeyCode::F(3))),
    key("F4", 4, KeyMatcher::Code(KeyCode::F(4))),
    gap(2),
    key("F5", 4, KeyMatcher::Code(KeyCode::F(5))),
    key("F6", 4, KeyMatcher::Code(KeyCode::F(6))),
    key("F7", 4, KeyMatcher::Code(KeyCode::F(7))),
    key("F8", 4, KeyMatcher::Code(KeyCode::F(8))),
    gap(2),
    key("F9", 4, KeyMatcher::Code(KeyCode::F(9))),
    key("F10", 5, KeyMatcher::Code(KeyCode::F(10))),
    key("F11", 5, KeyMatcher::Code(KeyCode::F(11))),
    key("F12", 5, KeyMatcher::Code(KeyCode::F(12))),
    gap(3),
    key("Prt", 5, KeyMatcher::Code(KeyCode::PrintScreen)),
    key("Scr", 5, KeyMatcher::Code(KeyCode::ScrollLock)),
    key("Pause", 7, KeyMatcher::Code(KeyCode::Pause)),
];

const NUMBER_ROW: &[KeyUnit] = &[
    key("`", 4, KeyMatcher::Chars("`~")),
    key("1", 4, KeyMatcher::Chars("1!")),
    key("2", 4, KeyMatcher::Chars("2@")),
    key("3", 4, KeyMatcher::Chars("3#")),
    key("4", 4, KeyMatcher::Chars("4$")),
    key("5", 4, KeyMatcher::Chars("5%")),
    key("6", 4, KeyMatcher::Chars("6^")),
    key("7", 4, KeyMatcher::Chars("7&")),
    key("8", 4, KeyMatcher::Chars("8*")),
    key("9", 4, KeyMatcher::Chars("9(")),
    key("0", 4, KeyMatcher::Chars("0)")),
    key("-", 4, KeyMatcher::Chars("-_")),
    key("=", 4, KeyMatcher::Chars("=+")),
    key("Bksp", 6, KeyMatcher::Code(KeyCode::Backspace)),
];

const QWERTY_ROW: &[KeyUnit] = &[
    key("Tab", 5, KeyMatcher::Code(KeyCode::Tab)),
    key("Q", 4, KeyMatcher::Chars("qQ")),
    key("W", 4, KeyMatcher::Chars("wW")),
    key("E", 4, KeyMatcher::Chars("eE")),
    key("R", 4, KeyMatcher::Chars("rR")),
    key("T", 4, KeyMatcher::Chars("tT")),
    key("Y", 4, KeyMatcher::Chars("yY")),
    key("U", 4, KeyMatcher::Chars("uU")),
    key("I", 4, KeyMatcher::Chars("iI")),
    key("O", 4, KeyMatcher::Chars("oO")),
    key("P", 4, KeyMatcher::Chars("pP")),
    key("[", 4, KeyMatcher::Chars("[{")),
    key("]", 4, KeyMatcher::Chars("]}")),
    key("\\", 4, KeyMatcher::Chars("\\|")),
];

const HOME_ROW: &[KeyUnit] = &[
    key("Caps", 6, KeyMatcher::Code(KeyCode::CapsLock)),
    key("A", 4, KeyMatcher::Chars("aA")),
    key("S", 4, KeyMatcher::Chars("sS")),
    key("D", 4, KeyMatcher::Chars("dD")),
    key("F", 4, KeyMatcher::Chars("fF")),
    key("G", 4, KeyMatcher::Chars("gG")),
    key("H", 4, KeyMatcher::Chars("hH")),
    key("J", 4, KeyMatcher::Chars("jJ")),
    key("K", 4, KeyMatcher::Chars("kK")),
    key("L", 4, KeyMatcher::Chars("lL")),
    key(";", 4, KeyMatcher::Chars(";:")),
    key("'", 4, KeyMatcher::Chars("'\"")),
    key("Enter", 7, KeyMatcher::NonKeypadCode(KeyCode::Enter)),
];

const BOTTOM_ROW: &[KeyUnit] = &[
    key("Shift", 7, KeyMatcher::Modifier(ModifierKeyCode::LeftShift)),
    key("Z", 4, KeyMatcher::Chars("zZ")),
    key("X", 4, KeyMatcher::Chars("xX")),
    key("C", 4, KeyMatcher::Chars("cC")),
    key("V", 4, KeyMatcher::Chars("vV")),
    key("B", 4, KeyMatcher::Chars("bB")),
    key("N", 4, KeyMatcher::Chars("nN")),
    key("M", 4, KeyMatcher::Chars("mM")),
    key(",", 4, KeyMatcher::Chars(",<")),
    key(".", 4, KeyMatcher::Chars(".>")),
    key("/", 4, KeyMatcher::Chars("/?")),
    key(
        "Shift",
        7,
        KeyMatcher::Modifier(ModifierKeyCode::RightShift),
    ),
];

const SPACE_ROW: &[KeyUnit] = &[
    key(
        "Ctrl",
        5,
        KeyMatcher::Modifier(ModifierKeyCode::LeftControl),
    ),
    key("Win", 5, KeyMatcher::Modifier(ModifierKeyCode::LeftSuper)),
    key("Alt", 5, KeyMatcher::Modifier(ModifierKeyCode::LeftAlt)),
    key("Space", 24, KeyMatcher::Code(KeyCode::Char(' '))),
    key("Alt", 5, KeyMatcher::Modifier(ModifierKeyCode::RightAlt)),
    key("Win", 5, KeyMatcher::Modifier(ModifierKeyCode::RightSuper)),
    key("Menu", 5, KeyMatcher::Code(KeyCode::Menu)),
    key(
        "Ctrl",
        5,
        KeyMatcher::Modifier(ModifierKeyCode::RightControl),
    ),
];

const NAV_TOP_ROW: &[KeyUnit] = &[
    key("Ins", 5, KeyMatcher::NonKeypadCode(KeyCode::Insert)),
    key("Home", 5, KeyMatcher::NonKeypadCode(KeyCode::Home)),
    key("PgUp", 5, KeyMatcher::NonKeypadCode(KeyCode::PageUp)),
];

const NAV_BOTTOM_ROW: &[KeyUnit] = &[
    key("Del", 5, KeyMatcher::NonKeypadCode(KeyCode::Delete)),
    key("End", 5, KeyMatcher::NonKeypadCode(KeyCode::End)),
    key("PgDn", 5, KeyMatcher::NonKeypadCode(KeyCode::PageDown)),
];

const ARROW_TOP_ROW: &[KeyUnit] = &[gap(6), key("Up", 5, KeyMatcher::NonKeypadCode(KeyCode::Up))];

const ARROW_BOTTOM_ROW: &[KeyUnit] = &[
    key("Left", 5, KeyMatcher::NonKeypadCode(KeyCode::Left)),
    key("Down", 5, KeyMatcher::NonKeypadCode(KeyCode::Down)),
    key("Right", 5, KeyMatcher::NonKeypadCode(KeyCode::Right)),
];

const NUMPAD_KEYS: &[PositionedKey] = &[
    PositionedKey {
        row: 1,
        col: 0,
        height: KEY_HEIGHT,
        key: spec("Num", 5, KeyMatcher::Code(KeyCode::NumLock)),
    },
    PositionedKey {
        row: 1,
        col: 6,
        height: KEY_HEIGHT,
        key: spec("/", 4, KeyMatcher::KeypadChars("/")),
    },
    PositionedKey {
        row: 1,
        col: 11,
        height: KEY_HEIGHT,
        key: spec("*", 4, KeyMatcher::KeypadChars("*")),
    },
    PositionedKey {
        row: 1,
        col: 16,
        height: KEY_HEIGHT,
        key: spec("-", 4, KeyMatcher::KeypadChars("-")),
    },
    PositionedKey {
        row: 2,
        col: 0,
        height: KEY_HEIGHT,
        key: spec(
            "7",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('7'), KeyCode::Home]),
        ),
    },
    PositionedKey {
        row: 2,
        col: 5,
        height: KEY_HEIGHT,
        key: spec(
            "8",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('8'), KeyCode::Up]),
        ),
    },
    PositionedKey {
        row: 2,
        col: 10,
        height: KEY_HEIGHT,
        key: spec(
            "9",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('9'), KeyCode::PageUp]),
        ),
    },
    PositionedKey {
        row: 2,
        col: 15,
        height: KEY_HEIGHT * 2,
        key: spec("+", 4, KeyMatcher::KeypadChars("+")),
    },
    PositionedKey {
        row: 3,
        col: 0,
        height: KEY_HEIGHT,
        key: spec(
            "4",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('4'), KeyCode::Left]),
        ),
    },
    PositionedKey {
        row: 3,
        col: 5,
        height: KEY_HEIGHT,
        key: spec(
            "5",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('5'), KeyCode::KeypadBegin]),
        ),
    },
    PositionedKey {
        row: 3,
        col: 10,
        height: KEY_HEIGHT,
        key: spec(
            "6",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('6'), KeyCode::Right]),
        ),
    },
    PositionedKey {
        row: 4,
        col: 0,
        height: KEY_HEIGHT,
        key: spec(
            "1",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('1'), KeyCode::End]),
        ),
    },
    PositionedKey {
        row: 4,
        col: 5,
        height: KEY_HEIGHT,
        key: spec(
            "2",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('2'), KeyCode::Down]),
        ),
    },
    PositionedKey {
        row: 4,
        col: 10,
        height: KEY_HEIGHT,
        key: spec(
            "3",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('3'), KeyCode::PageDown]),
        ),
    },
    PositionedKey {
        row: 4,
        col: 15,
        height: KEY_HEIGHT * 2,
        key: spec("Enter", 6, KeyMatcher::KeypadCodes(&[KeyCode::Enter])),
    },
    PositionedKey {
        row: 5,
        col: 0,
        height: KEY_HEIGHT,
        key: spec(
            "0",
            9,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('0'), KeyCode::Insert]),
        ),
    },
    PositionedKey {
        row: 5,
        col: 10,
        height: KEY_HEIGHT,
        key: spec(
            ".",
            4,
            KeyMatcher::KeypadCodes(&[KeyCode::Char('.'), KeyCode::Delete]),
        ),
    },
];

// ========== 绘制函数 ==========

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // 美化外框：双层边框 + 加粗标题
    let title_block = Block::default()
        .title(" Keyboard Check ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Rgb(80, 80, 80)));
    f.render_widget(title_block, area);

    let inner = area.inner(Margin {
        horizontal: 2,
        vertical: 2,
    });

    // 预留底部提示栏
    let legend_height = 1u16;
    let main_height = inner.height.saturating_sub(legend_height);

    let main_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: main_height,
    };

    let legend_area = Rect {
        x: inner.x,
        y: inner.y.saturating_add(main_height),
        width: inner.width,
        height: legend_height,
    };

    let exit_hint = if app.esc_exit_pending() {
        "ESC again to exit"
    } else {
        "double ESC to exit"
    };

    let legend = Paragraph::new(format!(
        " Press any key to highlight  {}  Ctrl+C to exit ",
        exit_hint
    ))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Rgb(120, 120, 120)));
    f.render_widget(legend, legend_area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(KEY_HEIGHT),
            Constraint::Length(KEY_HEIGHT),
            Constraint::Length(KEY_HEIGHT),
            Constraint::Length(KEY_HEIGHT),
            Constraint::Length(KEY_HEIGHT),
            Constraint::Length(KEY_HEIGHT),
        ])
        .split(main_area);

    draw_row(f, app, main_area, main_area.x, rows[0].y, FUNCTION_ROW);
    draw_row(f, app, main_area, main_area.x, rows[1].y, NUMBER_ROW);
    draw_row(f, app, main_area, main_area.x, rows[2].y, QWERTY_ROW);
    draw_row(f, app, main_area, main_area.x, rows[3].y, HOME_ROW);
    draw_row(f, app, main_area, main_area.x, rows[4].y, BOTTOM_ROW);
    draw_row(f, app, main_area, main_area.x, rows[5].y, SPACE_ROW);

    let nav_x = main_area.x.saturating_add(MAIN_NAV_GAP);
    draw_row(f, app, main_area, nav_x, rows[1].y, NAV_TOP_ROW);
    draw_row(f, app, main_area, nav_x, rows[2].y, NAV_BOTTOM_ROW);
    draw_row(f, app, main_area, nav_x, rows[4].y, ARROW_TOP_ROW);
    draw_row(f, app, main_area, nav_x, rows[5].y, ARROW_BOTTOM_ROW);

    let numpad_x = main_area.x.saturating_add(MAIN_NUMPAD_GAP);
    for positioned in NUMPAD_KEYS {
        draw_key(
            f,
            app,
            main_area,
            Rect {
                x: numpad_x.saturating_add(positioned.col),
                y: main_area
                    .y
                    .saturating_add(positioned.row.saturating_mul(KEY_HEIGHT)),
                width: positioned.key.width,
                height: positioned.height,
            },
            positioned.key,
        );
    }
}

fn draw_row(f: &mut Frame, app: &App, bounds: Rect, start_x: u16, y: u16, units: &[KeyUnit]) {
    let mut x = start_x;

    for unit in units {
        match unit {
            KeyUnit::Key(key) => {
                draw_key(
                    f,
                    app,
                    bounds,
                    Rect {
                        x,
                        y,
                        width: key.width,
                        height: KEY_HEIGHT,
                    },
                    *key,
                );
                x = x.saturating_add(key.width).saturating_add(1);
            }
            KeyUnit::Gap(width) => {
                x = x.saturating_add(*width);
            }
        }
    }
}

fn draw_key(f: &mut Frame, app: &App, bounds: Rect, area: Rect, key: KeySpec) {
    let max_x = bounds.x.saturating_add(bounds.width);
    let max_y = bounds.y.saturating_add(bounds.height);
    if area.x >= max_x || area.y >= max_y {
        return;
    }

    let width = area.width.min(max_x.saturating_sub(area.x));
    let height = area.height.min(max_y.saturating_sub(area.y));
    if width < 3 || height < 3 {
        return;
    }

    let is_pressed = key.matcher.is_pressed(app);
    let category = infer_category(key.matcher);
    let (bg, fg, border_color) = category.colors(is_pressed);

    // 3D 效果：未按下时圆润凸起，按下时厚重凹陷
    let border_type = if is_pressed {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };

    // 按下时文字加粗
    let style = if is_pressed {
        Style::default().bg(bg).fg(fg).add_modifier(Modifier::BOLD)
    } else {
        Style::default().bg(bg).fg(fg)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color))
        .style(style);

    let paragraph = Paragraph::new(key.label)
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(
        paragraph,
        Rect {
            x: area.x,
            y: area.y,
            width,
            height,
        },
    );
}

impl KeyMatcher {
    fn is_pressed(self, app: &App) -> bool {
        match self {
            KeyMatcher::Code(code) => app.is_pressed(code),
            KeyMatcher::NonKeypadCode(code) => app.is_pressed_non_keypad(code),
            KeyMatcher::Chars(chars) => chars
                .chars()
                .any(|ch| app.is_pressed_non_keypad(KeyCode::Char(ch))),
            KeyMatcher::KeypadChars(chars) => chars
                .chars()
                .any(|ch| keypad_code_pressed(app, KeyCode::Char(ch))),
            KeyMatcher::KeypadCodes(codes) => {
                codes.iter().any(|code| keypad_code_pressed(app, *code))
            }
            KeyMatcher::Modifier(modifier) => app.is_modifier_pressed(modifier),
        }
    }
}

fn keypad_code_pressed(app: &App, code: KeyCode) -> bool {
    app.is_pressed_on_keypad(code) || app.is_pressed_non_keypad(code)
}

#[cfg(test)]
fn full_size_key_count() -> usize {
    count_row_keys(FUNCTION_ROW)
        + count_row_keys(NUMBER_ROW)
        + count_row_keys(QWERTY_ROW)
        + count_row_keys(HOME_ROW)
        + count_row_keys(BOTTOM_ROW)
        + count_row_keys(SPACE_ROW)
        + count_row_keys(NAV_TOP_ROW)
        + count_row_keys(NAV_BOTTOM_ROW)
        + count_row_keys(ARROW_TOP_ROW)
        + count_row_keys(ARROW_BOTTOM_ROW)
        + NUMPAD_KEYS.len()
}

#[cfg(test)]
fn count_row_keys(units: &[KeyUnit]) -> usize {
    units
        .iter()
        .filter(|unit| matches!(unit, KeyUnit::Key(_)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    #[test]
    fn test_full_size_layout_has_104_keys() {
        assert_eq!(full_size_key_count(), 104);
    }

    #[test]
    fn test_shifted_digit_lights_number_key() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::SHIFT));

        assert!(KeyMatcher::Chars("1!").is_pressed(&app));
    }

    #[test]
    fn test_keypad_state_lights_numpad_without_main_key() {
        let mut app = App::new();
        app.handle_key_event(KeyEvent::new_with_kind_and_state(
            KeyCode::Char('7'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
            KeyEventState::KEYPAD,
        ));

        assert!(KeyMatcher::KeypadCodes(&[KeyCode::Char('7'), KeyCode::Home]).is_pressed(&app));
        assert!(!KeyMatcher::Chars("7&").is_pressed(&app));
    }
}
