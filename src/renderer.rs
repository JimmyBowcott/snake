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

struct Point {
    x: i32,
    y: i32,
}

pub struct TerminalRenderer {
    screen: Vec<Vec<char>>,
    width: i32,
    grid_start: Point,
    height: i32,
    scale_x: i32,
    scale_y: i32,
    grid_size: i32,
}

impl TerminalRenderer {
    pub fn new(grid_size: i32) -> Self {
        let (term_w, term_h) = size().unwrap();
        let width = term_w as i32;
        let height = term_h as i32;

        let scale_y = (width.min(height)) / grid_size;
        let scale_x = 2 * scale_y;
        let grid_w = grid_size * scale_x;
        let grid_h = grid_size * scale_y;
        let grid_start = Point {
            x: (width - grid_w) / 2,
            y: (height - grid_h) / 2,
        };

        let screen = vec![vec![' '; width as usize]; height as usize];
        let _ = crossterm::execute!(stdout(), Hide);

        Self {
            screen,
            width,
            height,
            grid_start,
            scale_x,
            scale_y,
            grid_size,
        }
    }

    pub fn draw_grid(&mut self) {
        let dark_square = '░';
        let light_square = '▒';

        for gy in 0..self.grid_size {
            for gx in 0..self.grid_size {
                let ch = if (gx + gy) % 2 == 0 {
                    dark_square
                } else {
                    light_square
                };
                self.put_char(gx, gy, ch);
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
        self.draw_grid();
    }

    fn put_char(&mut self, gx: i32, gy: i32, ch: char) {
        let start_x = self.grid_start.x + gx * self.scale_x;
        let start_y = self.grid_start.y + gy * self.scale_y;

        for y in 0..self.scale_y {
            for x in 0..self.scale_x {
                let sx = (start_x + x) as usize;
                let sy = (start_y + y) as usize;

                if sy < self.height as usize && sx < self.width as usize {
                    self.screen[sy][sx] = ch;
                }
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32) {}

    fn present(&mut self) {
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
