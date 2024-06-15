pub mod setup {
    use std::process::exit;

    use crate::math;
    use crate::window::Context;
    use sdl2::render::Canvas;
    pub use sdl2::video::Window;
    use sdl2::Sdl;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    pub trait State {
        fn update(&self, canvas: &mut Canvas<Window>, sdl_context: &Sdl) {
            
        }
        

        fn setup(&self, canvas: &mut Canvas<Window>, sdl_context: &Sdl) {
            
        }
    }

    pub fn start(sdl_context: &Sdl, canvas: &mut Canvas<Window>, state: Box<dyn State>) {
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        state.setup(canvas, sdl_context);
        'running: loop {
            state.update(canvas, sdl_context);
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
    
            canvas.present();
        }
    }
}

pub mod math {
    pub struct Vec2<T> {
        pub x: T,
        pub y: T
    }

    impl<T> Vec2<T> {
        pub fn new(x: T, y: T) -> Vec2<T> {
            return Vec2::<T>{x: x, y: y};
        }
    }

    pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
        Vec2::new(x, y)
    }
}

pub mod window {
    extern crate sdl2;

    pub use sdl2::render::Canvas;
    pub use sdl2::video::Window;
    use sdl2::Sdl;

    pub struct Context {}

    impl Context {
        pub fn new() -> Sdl {
            let sdl_context = sdl2::init().unwrap();
            sdl_context
        }

        pub fn new_canvas(sdl_context: &Sdl, title: &str, width: u32, height: u32) -> Canvas<Window> {
            let video_subsystem = sdl_context.video().unwrap();
        
            let window = video_subsystem.window(title, width, height)
                .position_centered()
                .build()
                .unwrap();
        
            let mut canvas = window.into_canvas().build().unwrap();
            canvas
        } 
    }
}


pub mod event {
    pub use sdl2::event::Event;
}

pub mod graphics {
    pub use sdl2::pixels::Color;
    pub use sdl2::render::Canvas;
    use sdl2::rect::Rect;

    use crate::math::*;
    pub struct Rectangle {
        pub position: Vec2<i32>,
        pub scale: Vec2<u32>
    }

    impl Rectangle {
        pub fn draw(&self) {
            let pos_x = self.position.x;
            let pos_y = self.position.y;
            let scale_x = self.scale.x;
            let scale_y = self.scale.y;
            let rect = Rect::new(pos_x, pos_y, scale_x, scale_y);
            
        }
    }
}

pub mod input {
    pub use sdl2::keyboard::Keycode;
}