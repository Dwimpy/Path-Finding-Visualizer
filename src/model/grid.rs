use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::model::cell::Cell;

pub struct Grid {
    cells: Vec<Cell>,
	thickness: i32,
	outline: bool,
}
impl Grid {
    pub fn new(start: Point, rows: usize, cols: usize) -> Result<Self, String> {
        let mut cells = Vec::with_capacity(rows * cols);
        for row in 0..rows {
            for col in 0..cols {
                let mut cell = Cell::new((start.x + col as i32) as i32, (start.y + row as i32) as i32, Color::RGBA(133, 24, 56, 0xFF)).unwrap();
				cell.adjust_to_grid(col as u32, row as u32);
                cells.push(cell);
            }
        }
		let thickness = 0;
		let outline = false;
        Ok(Grid { cells, thickness, outline })
    }

    pub fn draw (&self, canvas: &mut WindowCanvas)  {
        for cell in &self.cells {
            cell.draw(canvas, self.thickness, self.outline)
        }
	}
	pub fn set_outline(&mut self) {
		self.outline = true;
	}

	pub fn remove_outline(&mut self)  {
		self.outline = false;
	}


	pub fn set_thickness(&mut self, thickness: i32) {
		self.thickness = thickness;
	}
}