static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;

extern crate gl;
extern crate cgmath;
mod macros;
mod window;
mod game;
mod shader;
mod sprite;
mod texture;
mod game_object;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
