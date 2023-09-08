use sdl2::EventPump;
use crate::view;

pub mod renderer;
pub mod window;

pub struct View {
	renderer: view::renderer::Renderer,
}

impl View {
	pub fn new () -> Self {
		let window = view::window::Window::new().unwrap();
		let renderer = window.into_renderer();
		View { renderer }
	}

	pub fn run (mut self) -> () {
		self.renderer.run().unwrap()
	}
}