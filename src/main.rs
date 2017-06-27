extern crate glutin;
extern crate chrono;
extern crate float_duration;
extern crate game_time;
extern crate graphics;
extern crate gfx_graphics;

pub mod game;

fn main() {
    let mut game = game::Game::build_with_defaults();
    game.run();
}
