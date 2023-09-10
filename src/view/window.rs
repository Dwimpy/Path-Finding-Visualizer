extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{CanvasBuilder, WindowCanvas};
use sdl2::{EventPump, Sdl};
use crate::view::renderer::Renderer;
use crate::view::View;

pub struct Window {
	sdl_context: sdl2::Sdl,
	canvas_builder: CanvasBuilder
}

impl Window {
	pub fn new() -> Result <Window, String> {
		let sdl_context = sdl2::init().unwrap();

		let video_subsystem = sdl_context
			.video()
			.unwrap();

		let window = video_subsystem
			.window("Path Finding Visualizer", 1680, 960)
			.opengl()
			.position_centered()
			.borderless()
			.build()
			.unwrap();

		let canvas_builder = window.into_canvas();

		Ok(Window { sdl_context, canvas_builder})
	}
	pub fn into_canvas(self) -> (WindowCanvas, EventPump) {
		let canvas = self.canvas_builder.build().unwrap();
		let event = self.sdl_context.event_pump().unwrap();
		(canvas, event)
	}
}