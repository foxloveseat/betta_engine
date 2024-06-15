use betta::{
    graphics::*, math::{vec2, Vec2}, setup::*, window::*
};

fn main() {
    let sdl_context = Context::new();
    let mut canvas = Context::new_canvas(&sdl_context, "Game in betta", 800, 700);
    start(&sdl_context, &mut canvas, Box::<GameState>::new(GameState {}));
}


struct GameState {}

impl State for GameState {
    fn setup(&self, canvas: &mut Canvas<Window>, sdl_context: &sdl2::Sdl) {
        canvas.set_draw_color(Color::RGB(10, 52, 210));
    }
    fn update(&self, canvas: &mut Canvas<Window>, sdl_context: &sdl2::Sdl) {
        canvas.clear();
        let rect_pos = Vec2::new(400, 350);
        let rect_scale = Vec2::new(50, 50);
        let rect = Rectangle { position: rect_pos,  scale: rect_scale };
        rect.draw();
    }
    
}