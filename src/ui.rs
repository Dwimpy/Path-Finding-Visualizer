mod window;

use sdl2::render::WindowCanvas;
use crate::model::grid::Grid;
use crate::ui::window::Window;


pub struct UI {
	window: Window,
	grid: Grid,
}

impl UI {
	pub fn new (size: (u32, u32)) -> Self {
		let window = Window::new();
		let mut grid = Grid::new(64, 64, size).unwrap();
		grid.set_outline();
		grid.set_thickness(1);
		UI { window, grid}
	}

	pub fn draw(&self, canvas: &mut WindowCanvas) {
		self.grid.draw(canvas);
		self.window.render(canvas);
	}
}