use specs;
use cgmath::Vector2;

pub trait Component {
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position(Vector2<f64>);
#[derive(Debug, Clone, PartialEq)]
pub struct Movable(Vector2<f64>);

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

pub fn register_components(world: &mut specs::World) {
    world.register::<Position>();
    world.register::<Movable>();
}
