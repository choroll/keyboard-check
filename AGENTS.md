# Agent Instructions

## Project: keyboard-check

A Rust TUI application that visualizes keyboard input in the terminal. Shows a QWERTY keyboard layout and highlights keys when pressed.

## Tech Stack
- **Language**: Rust
- **TUI Framework**: ratatui + crossterm
- **License**: MIT

## Development Commands
- cargo run - Run the application
- cargo test - Run tests
- cargo clippy - Lint
- cargo fmt - Format
- cargo build --release - Release build

## Project Structure
- src/main.rs - Entry point, event loop
- src/app.rs - Application state and logic
- src/ui.rs - Keyboard rendering
- Cargo.toml - Dependencies
- README.md - Bilingual (Chinese/English)
- LICENSE - MIT License

## Key Conventions
- Use crossterm::event for non-blocking key input
- Key highlights depend on KeyEventKind: Press/Repeat inserts, Release removes
- Enable KeyboardEnhancementFlags::REPORT_EVENT_TYPES when supported so terminals can emit Release events
- Render keyboard layout using ratatui Block and Paragraph widgets
- Highlight pressed keys with color change (green background)
- Support both character keys and special keys (Enter, Space, etc.)
- Clean exit on Esc or Ctrl+C

## Git Workflow
- Standard Git open source project
- Commit frequently with descriptive messages
- Include both Chinese and English in README

## Testing
- Unit tests for key event handling
- Manual testing: run cargo run and press keys to verify highlighting
