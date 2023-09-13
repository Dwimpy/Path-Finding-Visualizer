use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::model::grid::{Grid, GridBuilder};

const BLACK_OUTLINE: Color = Color::RGBA(0, 0, 0, 0xFF);

pub struct UI {
	grid: Grid,
}

impl UI {
	pub fn new (margin: i32, size: (u32, u32)) -> Self {
		let mut grid = GridBuilder::new(size.0 as u32, (0.6 * size.1 as f32) as u32)
			.margin(margin as u32)
			.offset(0.2)
			.outline_thickness(1)
			.build()
			.unwrap();
		UI { grid }
	}

	pub fn draw(&mut self, canvas: &mut WindowCanvas) {
		self.draw_grid(canvas)
	}

	fn draw_grid(&mut self, canvas: &mut WindowCanvas) {
		let size = self.grid.size();
		let margin = self.grid.margin();
		let outline = self.grid.outline();
		let thickness = self.grid.thickness();
		let offset = self.grid.offset();
		let canvas_size = canvas.output_size().unwrap();
		let cells = self.grid.get_cells_mut();
		for cell in cells {
			let x_coord = margin as i32 + cell.x() * size;
			let y_coord = (canvas_size.1 as f32 * offset) as i32 + margin as i32 + cell.y() * size;
			canvas.set_draw_color(cell.color());
			canvas.fill_rect(Rect::new(x_coord, y_coord, size as u32, size as u32)).unwrap();
			canvas.set_draw_color(BLACK_OUTLINE);
			if outline {
				cell.draw_border(canvas, x_coord, y_coord, size as u32, thickness);
			}
		}
	}

}