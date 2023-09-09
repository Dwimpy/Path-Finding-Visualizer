extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{CanvasBuilder, WindowCanvas};
use sdl2::Sdl;
use crate::view::renderer::Renderer;

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
			.window("Path Findin Visualizer", 1920, 1080)
			.opengl()
			.position_centered()
			.borderless()
			.build()
			.unwrap();

		let canvas_builder = window.into_canvas();

		Ok(Window { sdl_context, canvas_builder})
	}

    pub fn into_renderer(self) -> Renderer {
        let canvas = self.canvas_builder.build().unwrap();
        let event_pump = self.sdl_context.event_pump().unwrap();
		Renderer::new(canvas, event_pump).unwrap()
    }
}