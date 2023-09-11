use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ WindowCanvas };

const BLACK_OUTLINE: Color = Color::RGBA(0, 0, 0, 0xFF);
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

    pub fn draw(self, canvas: &mut WindowCanvas, thickness: i32, has_border: bool) {

		let thickness = 1;
		let x1 = self.x;
		let x2 = self.x + self.width as i32;
		let y1 = self.y;
		let y2 = self.y + self.height as i32;

        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
        canvas.set_draw_color(BLACK_OUTLINE);
		if has_border {
			self.draw_border(canvas, thickness);
		}
    }

	fn draw_border(self, canvas: &mut WindowCanvas, thickness: i32) {

		let x1 = self.x;
		let x2 = self.x + self.width as i32;
		let y1 = self.y;
		let y2 = self.y + self.height as i32;

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