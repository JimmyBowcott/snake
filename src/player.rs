use std::{collections::VecDeque, vec};

use crate::renderer::Renderer;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Player {
    body: VecDeque<(i32, i32)>,
    direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Self {
            body: VecDeque::from(vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)]),
            direction: Direction::Right,
        }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        self.body.iter().for_each(|(x, y)| {
            renderer.put_char(*x, *y, '█');
        })
    }

    pub fn move_next_square(&mut self) -> Result<(), String> {
        let head = self.body.back().ok_or("No body found!")?;
        let next_square = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };
        self.body.push_back(next_square);
        self.body.pop_front();
        Ok(())
    }

    pub fn turn_up(&mut self) {
        self.turn(Direction::Up);
    }

    pub fn turn_down(&mut self) {
        self.turn(Direction::Down);
    }

    pub fn turn_left(&mut self) {
        self.turn(Direction::Left);
    }

    pub fn turn_right(&mut self) {
        self.turn(Direction::Right);
    }

    fn turn(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
