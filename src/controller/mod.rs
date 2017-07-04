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
        if !input.is_empty() {
            println!("{:?}", input);
        }

        const ACCEL_RATE: f64 = 50.0;
        const MAX_ACCEL: f64 = 250.0;

        let secs = time.elapsed_game_time().as_seconds();

        for evt in input.get_events() {
            match *evt {
                InputEvent::VirtKey(ref key, ref modi) => {
                    match *key {
                        VirtualKeyCode::Left => vel.x -= ACCEL_RATE * secs,
                        VirtualKeyCode::Right => vel.x += ACCEL_RATE * secs,
                        VirtualKeyCode::Up => vel.y -= ACCEL_RATE * secs,
                        VirtualKeyCode::Down => vel.y += ACCEL_RATE * secs,
                        _ => (),
                    }
                }
                _ => (),
            }
        }

    }
}
