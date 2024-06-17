use betta::{
    graphics::*, input::{self, *}, math::{vec2, Vec2}, setup::*, window::*, event::*
};


fn main() {
    let sdl_context = Context::new();
    let mut canvas = Context::new_canvas(&sdl_context, "Game in betta", 800, 700);
    let player = Player { position: Vec2::new(400, 350) };
    start(&sdl_context, &mut canvas, Box::<GameState>::new(GameState { player: player } ));
}


struct GameState {
    player: Player
}

impl State for GameState {

    fn update(&mut self, canvas: &mut Canvas<Window>, sdl_context: &sdl2::Sdl, event_pump: &mut sdl2::EventPump) {
        canvas.set_draw_color(Color::RGB(10, 52, 210));
        canvas.clear();
        
        let rect_pos = Vec2::new(self.player.position.x, self.player.position.y);
        let rect_scale = Vec2::new(50, 50);
        let rect = Rectangle { position: rect_pos,  scale: rect_scale,color: Color::RGB(52, 2, 2) };
        rect.draw(canvas);
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => exit(0),
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => self.player.position.y += 1,
                        _ => {}
                    }
                }
                _ => {},
            }
        }
    }
    
}



struct Player {
    position: Vec2<i32>,
}