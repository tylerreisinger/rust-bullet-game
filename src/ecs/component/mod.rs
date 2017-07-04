use specs;

pub mod pos;
pub mod render;
pub mod controller;

pub trait Component {
    fn name(&self) -> &str;
}

pub fn register_components(world: &mut specs::World) {
    world.register::<pos::Position>();
    world.register::<pos::Movable>();
    world.register::<render::Render>();
    world.register::<controller::Control>();
}
