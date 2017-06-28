use std::os;

use world;

use graphics::{self, Graphics, Transformed};
use opengl_graphics::{self, GlGraphics};
use gl;
use glutin;

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
        Game {
            window,
            evt_loop,
            is_running: false,
            world: world::World::new(),
            gl_context: None,
        }
    }

    pub fn create_window() -> (glutin::Window, glutin::EventsLoop) {
        let evt_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
            .with_dimensions(1200, 800)
            .with_title("Game")
            .build(&evt_loop)
            .expect("Unable to create window");

        unsafe { window.make_current() }.expect("Unable to activate OpenGL context.");

        (window, evt_loop)
    }

    pub fn run(&mut self) {
        let graphics = init_graphics(&mut self.window);
        self.gl_context = Some(graphics);
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

            self.update(&time);
            self.draw(&time);

            self.window.swap_buffers().unwrap();

            println!("{} -- {}", time.total_game_time(), time.total_wall_time());
            println!(
                "{} -- {}",
                time.elapsed_game_time(),
                time.elapsed_wall_time()
            );
            println!("{}", fps_counter.average_frame_rate());

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

    fn update(&mut self, time: &GameTime) {}

    fn draw(&mut self, time: &GameTime) {
        let mut gl_ctx = self.gl_context.as_mut().expect(
            "GlContext was not created properly!",
        );

        let viewport = Game::build_window_viewport(&self.window);

        gl_ctx.draw(viewport, |ctx, gl| {
            gl.clear_color([0.8, 0.8, 0.8, 1.0]);
            println!("{:?}", ctx.draw_state);

            let transform = ctx.transform;

            graphics::Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                [100.0, 100.0, 200.0, 200.0],
                &ctx.draw_state,
                transform,
                gl,
            );
        });
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
