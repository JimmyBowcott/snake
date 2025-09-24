use crossterm::{cursor::{self, Hide, Show}, queue, terminal::size};
use std::io::{Write, stdout};

pub trait Renderer {
    fn clear(&mut self);
    fn present(&self);
    fn put_char(&mut self, x: i32, y: i32, ch: char);
    fn draw_text(&mut self, text: &str, x: i32, y: i32);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub struct TerminalRenderer {
    width: i32,
    height: i32,
    screen: Vec<Vec<char>>,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        let (width, height) = size().unwrap();
        let width = width as i32;
        let height = height as i32;
        let screen = vec![vec![' '; width as usize]; height as usize];

        let _ = crossterm::execute!(stdout(), Hide);
        Self {
            width,
            height,
            screen,
        }
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = crossterm::execute!(stdout(), Show);
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&mut self) {
        for row in &mut self.screen {
            for ch in row.iter_mut() {
                *ch = ' ';
            }
        }
    }

    fn put_char(&mut self, x: i32, y: i32, ch: char) {
        if x < self.width && y < self.height {
            self.screen[y as usize][x as usize] = ch;
        }
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32) {
        if y >= self.height { return; }

        for (i, ch) in text.chars().enumerate() {
            let px = x + i as i32;
            if px >= self.width { break; }
            self.screen[y as usize][px as usize] = ch;
        }
    }

    fn present(&self) {
        let mut stdout = stdout();

        for (y, row) in self.screen.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                queue!(stdout, cursor::MoveTo(x as u16, y as u16)).unwrap();
                write!(stdout, "{}", ch).unwrap();
            }
        }

        stdout.flush().unwrap();
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}

