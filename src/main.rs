extern crate gl;
extern crate cgmath;
mod macros;
mod game;
mod shader;
mod sprite;
mod texture;
mod game_logic;

fn main() {
    let game = game::Game::init(800, 600);
}
