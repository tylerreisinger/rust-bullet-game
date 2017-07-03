use std::os;

use world;
use ecs::component;
use render;

use graphics;
use opengl_graphics::{self, GlGraphics};
use gl;
use glutin;
use specs::{self, Join};
use cgmath;

use game_time::{self, GameTime};

pub struct Game {
    window: glutin::Window,
    evt_loop: glutin::EventsLoop,
    is_running: bool,
    world: world::World,
    gl_context: Option<GlGraphics>,
}

impl Game {
    pub fn build_with_defaults() -> Game {
        let (window, evt_loop) = Game::create_window();
        let mut entity_set = specs::World::new();
        component::register_components(&mut entity_set);

        Game {
            window,
            evt_loop,
            is_running: false,
            world: world::World::new(entity_set),
            gl_context: None,
        }
    }

    pub fn world(&self) -> &world::World {
        &self.world
    }
    pub fn world_mut(&mut self) -> &mut world::World {
        &mut self.world
    }

    pub fn create_window() -> (glutin::Window, glutin::EventsLoop) {
        let window_dimensions = (1200, 800);
        let evt_loop = glutin::EventsLoop::new();

        let window = glutin::WindowBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
            .with_dimensions(window_dimensions.0, window_dimensions.1)
            .with_title("Game")
            .build(&evt_loop)
            .expect("Unable to create window");

        unsafe { window.make_current() }.expect("Unable to activate OpenGL context.");

        (window, evt_loop)
    }

    pub fn initialize(&mut self) {
        use render::{RectangleGraphic, RenderGraphicState};

        let graphics = init_graphics(&mut self.window);
        self.gl_context = Some(graphics);

        let entity_set = self.world.get_specs_mut();
        entity_set
            .create_entity()
            .with(component::pos::Position(cgmath::Vector2::new(50.0, 50.0)))
            .with(component::pos::Movable(cgmath::Vector2::new(0.0, 4.0)))
            .with(component::render::Render::new(
                RenderGraphicState::Rectangle(RectangleGraphic {
                    rect: graphics::Rectangle::new([1.0, 0.0, 1.0, 1.0]),
                    width: 25.0,
                    height: 25.0,
                }),
            ))
            .build();

        entity_set.maintain();
    }

    pub fn run(&mut self) {
        self.initialize();

        self.game_loop();
    }

    fn game_loop(&mut self) {
        use game_time::FrameCount;

        let mut clock = game_time::GameClock::new();
        let mut fps_counter =
            game_time::FrameCounter::new(30.0, game_time::framerate::LinearAverageSampler::new());

        self.is_running = true;

        while self.is_running {
            let time = clock.tick(&game_time::step::FixedStep::new(&fps_counter));
            fps_counter.tick(&time);

            self.handle_events();

            self.world
                .get_specs_mut()
                .add_resource_with_id(time.clone(), 0);

            self.update(&time);
            self.draw(&time);

            self.window.swap_buffers().unwrap();

            self.post_frame(&time, &fps_counter);

            clock.sleep_remaining(&fps_counter);
        }
    }

    fn handle_events(&mut self) {
        use glutin::{Event, WindowEvent};

        let mut is_running = true;
        self.evt_loop.poll_events(|evt| match evt {
            Event::WindowEvent { event: e, .. } => {
                match e {
                    WindowEvent::Closed => is_running = false,
                    _ => (),
                }
            }
        });

        self.is_running = is_running;
    }

    fn update(&mut self, time: &GameTime) {
        let mut dispatcher = specs::DispatcherBuilder::new()
            .add(component::pos::MovementSystem, "movement", &[])
            .build();
        dispatcher.dispatch(&mut self.world.get_specs_mut().res);

        self.world.get_specs_mut().maintain();
    }

    fn draw(&mut self, time: &GameTime) {
        let mut gl_ctx = self.gl_context.as_mut().unwrap();
        let mut world = &mut self.world;

        let viewport = Game::build_window_viewport(&self.window);
        //let mut specs = self.world.get_specs_mut();

        gl_ctx.draw(viewport, |ctx, gl| { render::render(world, &ctx, gl); });
    }

    fn post_frame<C: game_time::FrameCount>(&mut self, time: &GameTime, fps_counter: &C) {
        println!("{} -- {}", time.total_game_time(), time.total_wall_time());
        println!(
            "{} -- {}",
            time.elapsed_game_time(),
            time.elapsed_wall_time()
        );
        println!("{}", fps_counter.average_frame_rate());

        for e in self.world.get_specs().entities().join() {
            let pos_reader = self.world.get_specs().read::<component::pos::Position>();
            let pos = pos_reader.get(e).unwrap();
            println!(">>> Entity {} @ {:?}", e.id(), pos.0);
        }
    }

    fn build_window_viewport(window: &glutin::Window) -> graphics::Viewport {
        let window_size = window.get_inner_size_points().unwrap();
        let fb_size = window.get_inner_size_pixels().unwrap();

        graphics::Viewport {
            rect: [0, 0, window_size.0 as i32, window_size.1 as i32],
            draw_size: [fb_size.0 as u32, fb_size.1 as u32],
            window_size: [window_size.0 as u32, window_size.1 as u32],
        }
    }
}

fn init_graphics(window: &mut glutin::Window) -> GlGraphics {
    gl::load_with(|s| window.get_proc_address(s) as *const os::raw::c_void);

    GlGraphics::new(opengl_graphics::OpenGL::V3_2)
}
