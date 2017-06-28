use specs;

pub struct World {
    components: specs::World,
}

impl World {
    pub fn new(components: specs::World) -> World {
        World { components }
    }

    pub fn get_components(&self) -> &specs::World {
        &self.components
    }
    pub fn get_components_mut(&mut self) -> &mut specs::World {
        &mut self.components
    }
}
