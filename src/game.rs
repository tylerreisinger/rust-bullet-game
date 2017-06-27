use glutin;
use float_duration::FloatDuration;
use game_time;

pub struct Game {
    window: glutin::Window,
    evt_loop: glutin::EventsLoop,
    is_running: bool,
}

impl Game {
    pub fn build_with_defaults() -> Game {
        let (window, evt_loop) = Game::create_window();
        Game {
            window,
            evt_loop,
            is_running: false,
        }
    }

    pub fn create_window() -> (glutin::Window, glutin::EventsLoop) {
        let evt_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(1200, 800)
            .with_title("Game")
            .build(&evt_loop)
            .expect("Unable to create window");
        
        unsafe {
            window.make_current()
        }.expect("Unable to activate OpenGL context.");
        
        (window, evt_loop)
    }

    pub fn run(&mut self) {
        self.game_loop();
    }

    fn game_loop(&mut self) {
        use game_time::FrameCount;

        let mut clock = game_time::GameClock::new();
        let mut fps_counter = game_time::FrameCounter::new(
            30.0, game_time::framerate::LinearAverageSampler::new());

        self.is_running = true;

        while self.is_running {
            let time = clock.tick(&game_time::step::FixedStep::new(&fps_counter));
            fps_counter.tick(&time);

            self.handle_events();

            self.update();
            self.draw();

            self.window.swap_buffers().unwrap();
            println!("{} -- {}", time.total_game_time(), time.total_wall_time());
            println!("{} -- {}", time.elapsed_game_time(), time.elapsed_wall_time());
            println!("{}", fps_counter.average_frame_rate());

            clock.sleep_remaining(&fps_counter);
        }
    }

    fn handle_events(&mut self) {
        use glutin::{Event, WindowEvent};

        let mut is_running = true;
        self.evt_loop.poll_events(|evt| {
            match evt {
                Event::WindowEvent{event: e, ..} => {
                    match e {
                        WindowEvent::Closed => is_running = false,
                        _ => (),
                    }
                }
            }
        });

        self.is_running = is_running;
    }

    fn update(&mut self) {
    }

    fn draw(&mut self) {
    }
}
