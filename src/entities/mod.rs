use specs;

pub mod pos;

pub trait Component {
    fn name(&self) -> &str;
}

pub fn register_components(world: &mut specs::World) {
    world.register::<pos::Position>();
    world.register::<pos::Movable>();
}
