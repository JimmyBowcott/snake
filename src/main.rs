use std::io;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::renderer::TerminalRenderer;

mod game;
mod player;
mod renderer;
mod controller;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let grid_size = 24;
    let mut renderer = TerminalRenderer::new(grid_size);
    let mut game = game::Game::new(grid_size);

    let result = (|| {
        game.run(&mut renderer);
        Ok(())
    })();

    disable_raw_mode()?;
    result
}
