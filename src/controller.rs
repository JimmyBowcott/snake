use crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

pub struct InputController {
    current: Key,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            current: Key::Right,
        }
    }

    pub fn poll(&mut self) {
        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('w') => self.current = Key::Up,
                    KeyCode::Left | KeyCode::Char('a') => self.current = Key::Left,
                    KeyCode::Down | KeyCode::Char('s') => self.current = Key::Down,
                    KeyCode::Right | KeyCode::Char('d') => self.current = Key::Right,
                    KeyCode::Char('q') => self.current = Key::Quit,
                    _ => {}
                }
            }
        }
    }

    pub fn direction(&self) -> Key {
        self.current
    }

    pub fn should_exit(&self) -> bool {
        self.current == Key::Quit
    }
}

