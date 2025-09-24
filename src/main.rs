use std::io;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use renderer::Renderer;
use crate::renderer::TerminalRenderer;

mod game;
mod player;
mod renderer;
mod controller;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut renderer = TerminalRenderer::new();
    let mut game = game::Game::new(renderer.width(), renderer.height());

    let result = (|| {
        game.run(&mut renderer);
        Ok(())
    })();

    disable_raw_mode()?;
    result
}
