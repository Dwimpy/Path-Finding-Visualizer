use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::model::grid::Grid;
use crate::ui::UI;
use crate::view;

pub mod sdl_window;

pub struct View {
	canvas: WindowCanvas,
	event: EventPump,
	ui: UI,
}

impl View {
	pub fn new () -> Self {
		let window = view::sdl_window::Window::new().unwrap();
		let (canvas, event) = window.into_canvas();
		let ui = UI::new();
		View { canvas, event, ui }
	}

	pub fn render(&mut self) -> Result <(), String> {
		self.canvas.set_draw_color(Color::RGB(0, 255, 255));
		self.canvas.clear();
		self.canvas.present();
		let mut i = 0;

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
			self.ui.draw(&mut self.canvas);
			self.canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
		Ok(())
	}
}