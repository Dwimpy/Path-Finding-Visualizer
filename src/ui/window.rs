use sdl2::render::WindowCanvas;
use crate::ui::window::window_border::WindowBorder;

mod window_border;

pub struct Window {
	window_border: WindowBorder
}

impl Window {
	pub fn new() -> Self {
		let window_border = WindowBorder::new(24);
		Window { window_border }
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		self.window_border.render(canvas);
	}
}