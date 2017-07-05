use specs;
use cgmath::Vector2;
use glutin::VirtualKeyCode;
use game_time::GameTime;

use input::{InputManager, InputEvent};

pub trait Controller {
    fn do_actions(
        &mut self,
        entity: &mut specs::Entity,
        time: &GameTime,
        vel: &mut Vector2<f64>,
        input: &InputManager,
    );
}

pub struct HumanController {}

impl HumanController {
    pub fn new() -> HumanController {
        HumanController {}
    }
}

impl Controller for HumanController {
    fn do_actions(
        &mut self,
        entity: &mut specs::Entity,
        time: &GameTime,
        vel: &mut Vector2<f64>,
        input: &InputManager,
    ) {

    }
}
