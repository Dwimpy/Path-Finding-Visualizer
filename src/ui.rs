use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::model::grid::Grid;



pub struct UI {
	grid: Grid,
}

impl UI {
	pub fn new (margin: i32, size: (u32, u32)) -> Self {
		let mut grid = Grid::new(Point::new(margin, margin + (0.15 * size.1 as f32) as i32), Point::new((size.0 as i32) - margin, (1.0 * (size.1 as i32) as f32) as i32 - margin), 32).unwrap();
		grid.set_outline();
		grid.set_thickness(1);
		UI { grid }
	}

	pub fn draw(&self, canvas: &mut WindowCanvas) {
		self.grid.draw(canvas);
	}
}