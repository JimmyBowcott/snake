use std::{
    thread::sleep,
    time::{Duration, Instant},
};

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
        }
    }

    pub fn run(&mut self, renderer: &mut impl Renderer) -> Result<(), String> {
        self.running = true;
        loop {
            if !self.running {
                break;
            }

            let frame_start = Instant::now();

            self.handle_input();
            self.player.move_next_square()?;
            self.render(renderer);

            let elapsed = frame_start.elapsed();
            if elapsed < FRAME_DURATION_IN_MS {
                sleep(FRAME_DURATION_IN_MS - elapsed);
            }
        }
        Ok(())
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

    fn render(&self, renderer: &mut impl Renderer) {
        renderer.clear();
        self.player.draw(renderer);
        self.draw_score(renderer);
        renderer.present();
    }

    fn draw_score(&self, renderer: &mut impl Renderer) {
        renderer.draw_text(&format!("Score: {}", self.score), 2, 2);
    }
}

