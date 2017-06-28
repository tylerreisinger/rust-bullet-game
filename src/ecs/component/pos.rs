use specs::{self, ReadStorage, WriteStorage, Join};
use cgmath::Vector2;
use game_time::GameTime;

use ecs::component::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct Position(pub Vector2<f64>);
#[derive(Debug, Clone, PartialEq)]
pub struct Movable(pub Vector2<f64>);

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    pos: WriteStorage<'a, Position>,
    vel: ReadStorage<'a, Movable>,
    time: specs::Fetch<'a, GameTime>,
}
pub struct MovementSystem;

impl specs::Component for Position {
    type Storage = specs::VecStorage<Self>;
}

impl Component for Position {
    fn name(&self) -> &str {
        "Position"
    }
}

impl specs::Component for Movable {
    type Storage = specs::VecStorage<Self>;
}

impl Component for Movable {
    fn name(&self) -> &str {
        "Movable"
    }
}

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let time = &*data.time;
        for (pos, vel) in (&mut data.pos, &data.vel).join() {
            pos.0 += vel.0 * time.elapsed_game_time().as_seconds();
        }
    }
}
