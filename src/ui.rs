use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::model::grid::Grid;



pub struct UI {
	grid: Grid,
}

impl UI {
	pub fn new (size: (u32, u32)) -> Self {
		let mut grid = Grid::new(Point::new(10, 10), 128, 128).unwrap();
		grid.set_outline();
		grid.set_thickness(1);
		UI { grid }
	}

	pub fn draw(&self, canvas: &mut WindowCanvas) {
		self.grid.draw(canvas);
	}
}