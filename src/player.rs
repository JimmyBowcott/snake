use std::{collections::VecDeque, vec};

use crate::renderer::Renderer;

pub struct Player {
    pub body: VecDeque<(i32, i32)>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            body: VecDeque::from(vec![(5, 1), (4, 1), (3, 1), (2, 1), (1, 1)]),
        }
    }

    pub fn draw(&self, renderer: &mut impl Renderer) {
        self.body.iter().for_each(|(x, y)| {
            renderer.put_char(*x, *y, '*');
        })
    }
}
