use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ WindowCanvas };

const BLACK_OUTLINE: Color = Color::RGBA(0, 0, 0, 0xFF);
#[derive(Copy, Clone)]
pub struct Cell {
    x: i32,
    y: i32,
    color: Color,
}

impl Cell {
    pub fn new (x: i32, y: i32, color: Color) -> Result<Cell, String> {
        Ok(Cell { x, y, color })
    }

	pub fn x(&self) -> i32 {
		self.x
	}

	pub fn y(&self) -> i32 {
		self.y
	}

	pub fn color(&self) -> Color {
		self.color
	}

	pub fn draw_border(self, canvas: &mut WindowCanvas, x: i32, y: i32, cell_size: u32, thickness: u32) {
		let x1 = x;
		let x2 = x + cell_size as i32;
		let y1 = y;
		let y2 = y + cell_size as i32;
		for pixel in 0..thickness {
			canvas.draw_line(sdl2::rect::Point::new(x1, y1 + pixel as i32),
							 sdl2::rect::Point::new(x2, y1 + pixel as i32)).unwrap();

			canvas.draw_line(sdl2::rect::Point::new(x1, y2 - pixel as i32),
							 sdl2::rect::Point::new(x2, y2 - pixel as i32)).unwrap();

			canvas.draw_line(sdl2::rect::Point::new(x1 + pixel as i32, y1),
							 sdl2::rect::Point::new(x1 + pixel as i32, y2)).unwrap();

			canvas.draw_line(sdl2::rect::Point::new(x2 - pixel as i32, y1),
							 sdl2::rect::Point::new(x2 - pixel as i32, y2)).unwrap();
		}
	}

}