use std::{collections::VecDeque, vec};

use crate::renderer::Renderer;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    body: VecDeque<(i32, i32)>,
    direction: Direction,
    has_eaten: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            body: VecDeque::from(vec![(5, 1), (4, 1), (3, 1), (2, 1), (1, 1)]),
            direction: Direction::Right,
            has_eaten: false,
        }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        self.body.iter().for_each(|(x, y)| {
            renderer.put_char(*x, *y, '█');
        })
    }

    pub fn move_next_square(&mut self) {
        if let Some(head) = self.body.front() {
            let next_square = match self.direction {
                Direction::Up => (head.0, head.1 - 1),
                Direction::Down => (head.0, head.1 + 1),
                Direction::Left => (head.0 - 1, head.1),
                Direction::Right => (head.0 + 1, head.1),
            };
            self.body.push_front(next_square);

            if !self.has_eaten {
                self.body.pop_back();
            } else {
                self.has_eaten = false;
            }
        }
    }

    pub fn turn_up(&mut self) {
        if self.direction != Direction::Down {
            self.turn(Direction::Up);
        }
    }

    pub fn turn_down(&mut self) {
        if self.direction != Direction::Up {
            self.turn(Direction::Down);
        }
    }

    pub fn turn_left(&mut self) {
        if self.direction != Direction::Right {
            self.turn(Direction::Left);
        }
    }

    pub fn turn_right(&mut self) {
        if self.direction != Direction::Left {
            self.turn(Direction::Right);
        }
    }

    pub fn out_of_bounds(&self, grid_size: i32) -> bool {
        if let Some(head) = self.body.front() {
            if head.0 > grid_size - 1 || head.0 < 0 || head.1 > grid_size - 1 || head.1 < 0 {
                return true;
            }
        }
        false
    }

    pub fn collides_with_self(&self) -> bool {
        if let Some(head) = self.body.front() {
            for point in self.body.iter().skip(1) {
                if point.0 == head.0 && point.1 == head.1 {
                    return true;
                }
            }
        }
        false
    }

    pub fn body(&self) -> &VecDeque<(i32, i32)> {
        &self.body
    }

    pub fn collides(&self, x: i32, y: i32) -> bool {
        for point in self.body.iter() {
            if point.0 == x && point.1 == y {
                return true
            }
        }
        false
    }

    pub fn eat(&mut self) {
        self.has_eaten = true;
    }

    fn turn(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
