extern crate glutin;
extern crate chrono;
extern crate cgmath;
extern crate float_duration;
extern crate game_time;
extern crate graphics;
extern crate opengl_graphics;
extern crate gl;
extern crate specs;

pub mod game;
pub mod world;
pub mod entities;
pub mod render;

fn main() {
    let mut game = game::Game::build_with_defaults();
    game.run();
}
