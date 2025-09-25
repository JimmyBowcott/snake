use crossterm::{
    cursor::{self, Hide, Show},
    queue,
    terminal::size,
};
use std::io::{Write, stdout};

pub trait Renderer {
    fn clear(&mut self);
    fn present(&mut self);
    fn put_char(&mut self, x: i32, y: i32, ch: char);
    fn draw_text(&mut self, text: &str, x: i32, y: i32);
}

pub struct TerminalRenderer {
    width: i32,
    height: i32,
    scale_factor: i32,
    padding_left: i32,
    padding_right: i32,
    padding_top: i32,
    padding_bottom: i32,
    screen: Vec<Vec<char>>,
    grid_w: i32,
    grid_h: i32,
}

impl TerminalRenderer {
    pub fn new(grid_size: i32) -> Self {
        let (term_w, term_h) = size().unwrap();
        let width = term_w as i32;
        let height = term_h as i32;

        let scale_factor = (width.min(height)) / grid_size;
        let grid_w = grid_size * scale_factor;
        let grid_h = grid_size * scale_factor;
        let padding_left = (width - grid_w) / 2;
        let padding_right = width - grid_w - padding_left;
        let padding_top = (height - grid_h) / 2;
        let padding_bottom = height - grid_h - padding_top;

        let screen = vec![vec![' '; width as usize]; height as usize];
        let _ = crossterm::execute!(stdout(), Hide);

        Self {
            width,
            height,
            scale_factor,
            padding_left,
            padding_right,
            padding_top,
            padding_bottom,
            screen,
            grid_w,
            grid_h,
        }
    }

    pub fn draw_padding(&mut self) {
        for y in 0..self.padding_top {
            for x in 0..self.width {
                self.screen[y as usize][x as usize] = '█';
            }
        }

        for y in (self.height - self.padding_bottom)..self.height {
            for x in 0..self.width {
                self.screen[y as usize][x as usize] = '█';
            }
        }

        for y in 0..self.height {
            for x in 0..self.padding_left {
                self.screen[y as usize][x as usize] = '█';
            }
        }

        for y in 0..self.height {
            for x in (self.width - self.padding_right)..self.width {
                self.screen[y as usize][x as usize] = '█';
            }
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
        self.screen[y as usize][x as usize] = ch;
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32) {}

    fn present(&mut self) {
        self.draw_padding();
        let mut stdout = stdout();

        for (y, row) in self.screen.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                queue!(stdout, cursor::MoveTo(x as u16, y as u16)).unwrap();
                write!(stdout, "{}", ch).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}
