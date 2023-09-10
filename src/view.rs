use std::rc::Rc;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::model::grid::Grid;
use crate::view;
use crate::view::renderer::Renderer;

pub mod renderer;
pub mod window;

pub struct View {
	canvas: WindowCanvas,
	event: EventPump,
	renderer:Renderer,
}

impl View {
	pub fn new () -> Self {
		let window = view::window::Window::new().unwrap();
		let (mut canvas, event) = window.into_canvas();
		let renderer = Renderer::new();
		View { canvas, event, renderer }
	}

	pub fn render(&mut self) -> Result <(), String> {
		self.canvas.set_draw_color(Color::RGB(0, 255, 255));
		self.canvas.clear();
		self.canvas.present();
		let mut i = 0;
		let mut grid = Grid::new(64, 64).unwrap();
		grid.set_outline();
		grid.set_thickness(1);
		'running: loop {
			i = (i + 1) % 255;
			self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
			self.canvas.clear();
				for event in self.event.poll_iter() {
					match event {
						Event::Quit {..} |
						Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
							break 'running
						},
						_ => {}
					}
				}
			// The rest of the game loop goes here...
			grid.draw(&mut self.canvas);
			self.canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
		Ok(())
	}
}