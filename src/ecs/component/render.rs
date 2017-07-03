use specs::{self, ReadStorage, Join};
use graphics;
use opengl_graphics::{self, GlGraphics};
use cgmath;
use game_time::GameTime;
use render::RenderGraphicState;

use ecs::component::Component;

pub trait Renderer: Send + Sync {
    fn render(
        &self,
        entity: &specs::Entity,
        pso: cgmath::Vector2<f64>,
        obj: &Render,
        ctx: &mut graphics::Context,
        gl: &mut GlGraphics,
    );
}

#[derive(Debug, Clone)]
pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    pos: ReadStorage<'a, super::pos::Position>,
    renderer: ReadStorage<'a, Render>,
    entities: specs::Entities<'a>,
    time: specs::Fetch<'a, GameTime>,
    ctx: specs::Fetch<'a, graphics::Context>,
}

pub struct Render {
    pub state: RenderGraphicState,
}

impl Render {
    pub fn new(state: RenderGraphicState) -> Render {
        Render { state }
    }
}

impl specs::Component for Render {
    type Storage = specs::VecStorage<Self>;
}

impl Component for Render {
    fn name(&self) -> &str {
        "Render"
    }
}
