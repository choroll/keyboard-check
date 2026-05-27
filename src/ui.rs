use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();
    f.render_widget(
        Block::default()
            .title("Key Check - Press any key (ESC or Ctrl+C to exit)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
        area,
    );

    let inner = area.inner(Margin {
        horizontal: 2,
        vertical: 2,
    });

    let keyboard_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Function keys
            Constraint::Length(3), // Number row
            Constraint::Length(3), // QWERTY row
            Constraint::Length(3), // ASDF row
            Constraint::Length(3), // ZXCV row
            Constraint::Length(3), // Space bar
        ])
        .split(inner);

    // Function keys row
    draw_key_row(
        f,
        app,
        keyboard_area[0],
        &[
            ("Esc", false),
            ("F1", false),
            ("F2", false),
            ("F3", false),
            ("F4", false),
            ("F5", false),
            ("F6", false),
            ("F7", false),
            ("F8", false),
            ("F9", false),
            ("F10", false),
            ("F11", false),
            ("F12", false),
        ],
    );

    // Number row
    draw_key_row(
        f,
        app,
        keyboard_area[1],
        &[
            ("", false),
            ("1", false),
            ("2", false),
            ("3", false),
            ("4", false),
            ("5", false),
            ("6", false),
            ("7", false),
            ("8", false),
            ("9", false),
            ("0", false),
            ("-", false),
            ("=", false),
            ("Backspace", false),
        ],
    );

    // QWERTY row
    draw_key_row(
        f,
        app,
        keyboard_area[2],
        &[
            ("Tab", false),
            ("Q", false),
            ("W", false),
            ("E", false),
            ("R", false),
            ("T", false),
            ("Y", false),
            ("U", false),
            ("I", false),
            ("O", false),
            ("P", false),
            ("[", false),
            ("]", false),
            ("\\", false),
        ],
    );

    // ASDF row
    draw_key_row(
        f,
        app,
        keyboard_area[3],
        &[
            ("Caps", false),
            ("A", false),
            ("S", false),
            ("D", false),
            ("F", false),
            ("G", false),
            ("H", false),
            ("J", false),
            ("K", false),
            ("L", false),
            (";", false),
            ("'", false),
            ("Enter", false),
        ],
    );

    // ZXCV row
    draw_key_row(
        f,
        app,
        keyboard_area[4],
        &[
            ("Shift", false),
            ("Z", false),
            ("X", false),
            ("C", false),
            ("V", false),
            ("B", false),
            ("N", false),
            ("M", false),
            (",", false),
            (".", false),
            ("/", false),
            ("Shift", false),
        ],
    );

    // Space bar row
    draw_key_row(
        f,
        app,
        keyboard_area[5],
        &[
            ("Ctrl", false),
            ("Win", false),
            ("Alt", false),
            ("Space", true),
            ("Alt", false),
            ("Win", false),
            ("Menu", false),
            ("Ctrl", false),
        ],
    );
}

fn draw_key_row(f: &mut Frame, app: &App, area: Rect, keys: &[(&str, bool)]) {
    let constraints: Vec<Constraint> = keys
        .iter()
        .map(|(label, wide)| {
            if *wide {
                Constraint::Length(12)
            } else {
                Constraint::Length((label.len() as u16 + 3).max(4))
            }
        })
        .collect();

    let key_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .margin(0)
        .split(area);

    for (i, (label, _)) in keys.iter().enumerate() {
        if i < key_areas.len() {
            let is_pressed = is_key_pressed(app, label);
            let style = if is_pressed {
                Style::default().bg(Color::Green).fg(Color::Black)
            } else {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray))
                .style(style);

            let paragraph = Paragraph::new(*label)
                .alignment(Alignment::Center)
                .block(block);

            f.render_widget(paragraph, key_areas[i]);
        }
    }
}

fn is_key_pressed(app: &App, label: &str) -> bool {
    match label {
        "Esc" => app.is_pressed(crossterm::event::KeyCode::Esc),
        "Enter" => app.is_pressed(crossterm::event::KeyCode::Enter),
        "Space" => app.is_pressed(crossterm::event::KeyCode::Char(' ')),
        "Backspace" => app.is_pressed(crossterm::event::KeyCode::Backspace),
        "Tab" => app.is_pressed(crossterm::event::KeyCode::Tab),
        "Caps" => false,  // Simplified for demo
        "Shift" => false, // Simplified for demo
        "Ctrl" => false,  // Simplified for demo
        "Alt" => false,   // Simplified for demo
        "Win" => false,   // Simplified for demo
        "Menu" => false,  // Simplified for demo
        "F1" => app.is_pressed(crossterm::event::KeyCode::F(1)),
        "F2" => app.is_pressed(crossterm::event::KeyCode::F(2)),
        "F3" => app.is_pressed(crossterm::event::KeyCode::F(3)),
        "F4" => app.is_pressed(crossterm::event::KeyCode::F(4)),
        "F5" => app.is_pressed(crossterm::event::KeyCode::F(5)),
        "F6" => app.is_pressed(crossterm::event::KeyCode::F(6)),
        "F7" => app.is_pressed(crossterm::event::KeyCode::F(7)),
        "F8" => app.is_pressed(crossterm::event::KeyCode::F(8)),
        "F9" => app.is_pressed(crossterm::event::KeyCode::F(9)),
        "F10" => app.is_pressed(crossterm::event::KeyCode::F(10)),
        "F11" => app.is_pressed(crossterm::event::KeyCode::F(11)),
        "F12" => app.is_pressed(crossterm::event::KeyCode::F(12)),
        _ => {
            if label.len() == 1
                && let Some(ch) = label.chars().next()
            {
                let lower = ch.to_lowercase().next().unwrap_or(ch);
                return app.is_pressed(crossterm::event::KeyCode::Char(ch))
                    || app.is_pressed(crossterm::event::KeyCode::Char(lower));
            }
            false
        }
    }
}
