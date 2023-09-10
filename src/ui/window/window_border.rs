use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect;
use crate::ui::window::window_border::CornerDirection::{LowerLeft, LowerRight, UpperLeft, UpperRight};

pub enum CornerDirection {
	UpperLeft,
	LowerLeft,
	UpperRight,
	LowerRight,
}

pub enum BorderDirection {
	Up,
	Down,
	Left,
	Right
}

pub struct WindowBorder {
	corner_radius: u32,
	thickness: u32,
	color: Color,
}

impl WindowBorder {
	pub fn new (corner_radius: u32) -> Self {
		let color = Color::RGBA(34, 37, 41, 0x2F);
		let thickness = corner_radius;
		WindowBorder { corner_radius, thickness, color}
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		canvas.set_draw_color(Color::RGB(34, 37, 41));
		self.draw_border(canvas);
		// canvas.filled_circle(self.corner_radius as i16, self.corner_radius as i16, self.corner_radius as i16, Color::RGB(25, 25, 25)).unwrap();
		// canvas.filled_circle((size.0 - self.corner_radius) as i16, self.corner_radius as i16, self.corner_radius as i16, Color::RGB(25, 25, 25)).unwrap();
		// canvas.filled_circle(self.corner_radius as i16, self.corner_radius as i16, self.corner_radius as i16, Color::RGB(25, 25, 25)).unwrap();
		// canvas.filled_circle((size.0 - self.corner_radius) as i16, (size.1 - self.corner_radius) as i16, self.corner_radius as i16, Color::RGB(25, 25, 25)).unwrap();
		// canvas.fill_rect(Rect::new(self.corner_radius as i32, 0, size.0 - 2 * self.corner_radius, self.thickness)).unwrap();

		// canvas.fill_rect(Rect::new(self.corner_radius as i32, (size.1 - self.thickness) as i32, size.0 - 2 * self.corner_radius - 3, self.thickness)).unwrap();
		// canvas.fill_rect(Rect::new(0, self.corner_radius as i32, self.corner_radius + 3, size.1 - self.corner_radius)).unwrap();
		// canvas.fill_rect(Rect::new((size.0 - self.corner_radius - 3) as i32, self.corner_radius as i32, self.corner_radius + 3, size.1 - self.corner_radius)).unwrap();
	}

	fn draw_border(&self, canvas: &mut WindowCanvas) {
		let size = canvas.output_size().unwrap();
		self.draw_border_corner(canvas, UpperLeft);
		self.draw_border_corner(canvas, LowerLeft);
		self.draw_border_corner(canvas, UpperRight);
		self.draw_border_corner(canvas, LowerRight);
		canvas.fill_rect(Rect::new(self.corner_radius as i32, 1, (size.0 - 2 * self.thickness) as u32, 16)).unwrap();
	}

	fn draw_border_corner(&self, canvas: &mut WindowCanvas, dir: CornerDirection) {
		let size = canvas.output_size().unwrap();
		match dir {
			CornerDirection::UpperLeft => {self.draw_upper_left(canvas, self.corner_radius as i32, self.corner_radius as i32)}
			CornerDirection::LowerLeft => {self.draw_lower_left(canvas, self.corner_radius as i32, (size.1 - self.corner_radius) as i32)}
			CornerDirection::UpperRight => {self.draw_upper_right(canvas, (size.0 - self.corner_radius) as i32, self.corner_radius as i32)}
			CornerDirection::LowerRight => {self.draw_lower_right(canvas, (size.0 - self.corner_radius) as i32, (size.1 - self.corner_radius) as i32)}
		}
	}
	fn draw_upper_left(&self, canvas: &mut WindowCanvas, cx: i32, cy: i32) {
		let x = cx - self.corner_radius as i32;
		let y = cy - self.corner_radius as i32;
		for elem_x in x..cx {
			for elem_y in y..cy {
				let distance_squared = (elem_x - cx).pow(2) + (elem_y - cy).pow(2);
				if distance_squared < (self.corner_radius * self.corner_radius) as i32 {
					canvas.draw_point(rect::Point::new(elem_x, elem_y)).unwrap();
				}
			}
		}
	}

		fn draw_lower_left(&self, canvas: &mut WindowCanvas, cx: i32, cy: i32) {
		let x = cx - self.corner_radius as i32;
		let y = cy + self.corner_radius as i32;
		for elem_x in x..cx {
			for elem_y in cy..y {
				let distance_squared = (elem_x - cx).pow(2) + (elem_y - cy).pow(2);
				if distance_squared < (self.corner_radius * self.corner_radius) as i32 {
					canvas.draw_point(rect::Point::new(elem_x, elem_y)).unwrap();
				}
			}
		}
	}

		fn draw_upper_right(&self, canvas: &mut WindowCanvas, cx: i32, cy: i32) {
		let x = cx + self.corner_radius as i32;
		let y = cy - self.corner_radius as i32;
		for elem_x in cx..x {
			for elem_y in y..cy {
				let distance_squared = (elem_x - cx).pow(2) + (elem_y - cy).pow(2);
				if distance_squared < (self.corner_radius * self.corner_radius) as i32 {
					canvas.draw_point(rect::Point::new(elem_x, elem_y)).unwrap();
				}
			}
		}
	}

		fn draw_lower_right(&self, canvas: &mut WindowCanvas, cx: i32, cy: i32) {
		let x = cx + self.corner_radius as i32;
		let y = cy + self.corner_radius as i32;
		for elem_x in cx..x {
			for elem_y in cy..y {
				let distance_squared = (elem_x - cx).pow(2) + (elem_y - cy).pow(2);
				if distance_squared < (self.corner_radius * self.corner_radius) as i32 {
					canvas.draw_point(rect::Point::new(elem_x, elem_y)).unwrap();
				}
			}
		}
	}

}