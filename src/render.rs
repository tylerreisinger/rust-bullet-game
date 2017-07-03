use specs::{self, Join};

use graphics::{self, Graphics};
use opengl_graphics::GlGraphics;

use world::World;
use ecs::component;
use cgmath;

pub struct RenderGraphic {
    entity: specs::Entity,
    pos: cgmath::Vector2<f64>,
    state: RenderGraphicState,
}

#[derive(Clone)]
pub struct RectangleGraphic {
    pub rect: graphics::Rectangle,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub enum RenderGraphicState {
    Rectangle(RectangleGraphic),
}

pub fn render(world: &mut World, ctx: &graphics::Context, gl: &mut GlGraphics) {
    gl.clear_color([0.8, 0.8, 0.8, 1.0]);

    let specs = world.get_specs_mut();

    let transform = ctx.transform;

    let renderers = specs.read::<component::render::Render>();
    let positions = specs.read::<component::pos::Position>();
    let entities = specs.entities();

    for (entity, render, &component::pos::Position(ref pos)) in
        (&*entities, &renderers, &positions).join()
    {
        match render.state {
            RenderGraphicState::Rectangle(ref state) => {
                render_rect(&entity, render, pos, state, ctx, gl);
            }
        }
    }
}

fn render_rect(
    entity: &specs::Entity,
    renderer: &component::render::Render,
    pos: &cgmath::Vector2<f64>,
    state: &RectangleGraphic,
    ctx: &graphics::Context,
    gl: &mut GlGraphics,
) {
    let transform = ctx.transform;
    state.rect.draw(
        [pos.x, pos.y, state.width, state.height],
        &ctx.draw_state,
        transform,
        gl,
    );
}
