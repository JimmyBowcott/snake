use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    controller::InputController,
    player::Player,
    renderer::Renderer,
};

pub struct Game {
    running: bool,
    score: usize,
    player: Player,
    controller: InputController,
    screen_width: i32,
    screen_height: i32,
}

const FRAME_DURATION_IN_MS: Duration = Duration::from_millis(16);

impl Game {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        Game {
            running: true,
            score: 0,
            player: Player::new(screen_width.saturating_sub(2), screen_height / 2),
            controller: InputController::new(),
            screen_width,
            screen_height,
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

