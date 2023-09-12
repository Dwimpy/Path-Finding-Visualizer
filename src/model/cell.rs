use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ WindowCanvas };

const BLACK_OUTLINE: Color = Color::RGBA(0, 0, 0, 0xFF);
#[derive(Copy, Clone)]
pub struct Cell {
    x: i32,
    y: i32,
    cell_size: u32,
    color: Color,
}

impl Cell {
    pub fn new (x: i32, y: i32, color: Color) -> Result<Cell, String> {
		let cell_size = 20;
        Ok(Cell { x, y, cell_size, color })
    }

	pub fn adjust_to_grid(&mut self, col: u32, row: u32) {
		self.set_x(col);
		self.set_y(row);
	}
	fn set_x(&mut self, col: u32) {
		self.x += col as i32 * self.cell_size as i32
	}

	fn set_y(&mut self, row: u32) {
		self.y += row as i32 * self.cell_size as i32;
	}
    pub fn draw(self, canvas: &mut WindowCanvas, thickness: i32, has_border: bool) {

        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y, self.cell_size, self.cell_size)).unwrap();
        canvas.set_draw_color(BLACK_OUTLINE);
		if has_border {
			self.draw_border(canvas, thickness);
		}
    }

	fn draw_border(self, canvas: &mut WindowCanvas, thickness: i32) {

		let x1 = self.x;
		let x2 = self.x + self.cell_size as i32;
		let y1 = self.y;
		let y2 = self.y + self.cell_size as i32;

		for pixel in 0..thickness {
			canvas.draw_line(sdl2::rect::Point::new(x1, y1 + pixel),
							 sdl2::rect::Point::new(x2, y1 + pixel)).unwrap();

			canvas.draw_line(sdl2::rect::Point::new(x1, y2 - pixel),
							 sdl2::rect::Point::new(x2, y2 - pixel)).unwrap();

			canvas.draw_line(sdl2::rect::Point::new(x1 + pixel, y1),
							 sdl2::rect::Point::new(x1 + pixel, y2)).unwrap();

			canvas.draw_line(sdl2::rect::Point::new(x2 - pixel, y1),
							 sdl2::rect::Point::new(x2 - pixel, y2)).unwrap();
		}
	}
}