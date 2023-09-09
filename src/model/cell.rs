use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{CanvasBuilder, WindowCanvas};

const BLACK_OUTLINE: Color = Color::RGB(0, 0, 0);
#[derive(Copy, Clone)]
pub struct Cell {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
}

impl Cell {
    pub fn new (x: i32, y: i32, width: u32, height: u32, color: Color) -> Result<Cell, String> {
        Ok(Cell { x, y, width, height, color })
    }

    pub fn draw(self, canvas: &mut WindowCanvas ) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
        // canvas.set_draw_color(BLACK_OUTLINE);
        canvas.draw_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
    }
}