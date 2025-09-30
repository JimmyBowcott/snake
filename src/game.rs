use std::{
    collections::HashSet, thread::sleep, time::{Duration, Instant}
};
use rand::Rng;

use crate::{
    controller::{InputController, Key},
    player::Player,
    renderer::Renderer,
};

pub struct Game {
    running: bool,
    score: usize,
    player: Player,
    controller: InputController,
    grid_size: i32,
    apple: Option<(i32, i32)>,
}

const FRAME_DURATION_IN_MS: Duration = Duration::from_millis(200);

impl Game {
    pub fn new(grid_size: i32) -> Self {
        Game {
            grid_size,
            running: true,
            score: 0,
            player: Player::new(),
            controller: InputController::new(),
            apple: None,
        }
    }

    pub fn run(&mut self, renderer: &mut impl Renderer) {
        self.running = true;
        loop {
            if !self.running {
                break;
            }

            let frame_start = Instant::now();

            self.handle_input();
            self.player.move_next_square();
            if self.player.out_of_bounds(self.grid_size) || self.player.collides_with_self() {
                break;
            }

            if let Some(pos) = self.apple && self.player.collides(pos.0, pos.1) {
                self.player.eat();
                self.apple = None;
            }

            if self.apple.is_none() {
                self.spawn_apple();
            }
            self.render(renderer);

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION_IN_MS {
                sleep(FRAME_DURATION_IN_MS - elapsed);
            }
        }
    }

    pub fn handle_input(&mut self) {
        self.controller.poll();

        if self.controller.should_exit() {
            self.running = false;
            return;
        }

        match self.controller.direction() {
            Key::Up => self.player.turn_up(),
            Key::Down => self.player.turn_down(),
            Key::Left => self.player.turn_left(),
            Key::Right => self.player.turn_right(),
            _ => {}
        }
    }

    fn spawn_apple(&mut self) {
        let mut rng = rand::rng();
        let occupied: HashSet<_> = self.player.body().iter().cloned().collect();

        let free_cells: Vec<(i32, i32)> = (0..self.grid_size)
            .flat_map(|x| (0..self.grid_size).map(move |y| (x, y)))
            .filter(|pos| !occupied.contains(pos))
            .collect();

        if !free_cells.is_empty() {
            self.apple = Some(free_cells[rng.random_range(0..free_cells.len())]);
        }
    }

    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();
        self.player.draw(renderer);
        self.draw_score(renderer);
        self.draw_apple(renderer);
        renderer.present();
    }

    fn draw_score(&self, renderer: &mut impl Renderer) {
        renderer.draw_text(&format!("Score: {}", self.score), 2, 2);
    }

    fn draw_apple(&self, renderer: &mut impl Renderer) {
        if let Some(pos) = self.apple {
            renderer.put_char(pos.0, pos.1, '█');
        }
    }
}
