use specs::{self, ReadStorage, WriteStorage, Join};
use game_time::GameTime;

use ecs::{component, Component};
use controller::Controller;
use input::InputManager;

pub struct Control {
    controller: Box<Controller + Send + Sync>,
}

#[derive(Debug, Clone)]
pub struct ControlSystem;

#[derive(SystemData)]
pub struct ControlSystemData<'a> {
    pos: ReadStorage<'a, component::pos::Position>,
    vel: WriteStorage<'a, component::pos::Movable>,
    controller: WriteStorage<'a, Control>,
    entity: specs::Entities<'a>,
    time: specs::Fetch<'a, GameTime>,
    input: specs::Fetch<'a, InputManager>,
}

impl Control {
    pub fn new(controller: Box<Controller + Send + Sync>) -> Control {
        Control { controller }
    }
}

impl Component for Control {
    fn name(&self) -> &str {
        "Control"
    }
}

impl specs::Component for Control {
    type Storage = specs::VecStorage<Self>;
}

impl<'a> specs::System<'a> for ControlSystem {
    type SystemData = ControlSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let time = &*data.time;
        let input = &*data.input;

        for (mut entity, controller, pos, mut vel) in
            (
                &*data.entity,
                &mut data.controller,
                &data.pos,
                &mut data.vel,
            ).join()
        {
            controller
                .controller
                .do_actions(&mut entity, &time, &mut vel.0, &input);
        }
    }
}
