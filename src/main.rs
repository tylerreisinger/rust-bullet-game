extern crate glutin;
extern crate winit;
extern crate chrono;
extern crate cgmath;
extern crate float_duration;
extern crate game_time;
extern crate graphics;
extern crate opengl_graphics;
extern crate gl;
extern crate specs;
extern crate shred;
#[macro_use]
extern crate shred_derive;

pub mod game;
pub mod world;
pub mod ecs;
pub mod render;
pub mod input;
pub mod controller;

fn main() {
    let mut game = game::Game::build_with_defaults();
    game.run();
}
