use std::{collections::VecDeque, vec};

use crate::renderer::Renderer;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub body: VecDeque<(i32, i32)>,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
            body: VecDeque::from(vec![(5, 1), (4, 1), (3, 1), (2, 1), (1, 1)]),
        }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        self.body.iter().for_each(|(x, y)| {
            renderer.put_char(*x, *y, '*');
        })
    }
}
