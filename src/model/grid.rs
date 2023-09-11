use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::model::cell::Cell;

pub struct Grid {
    cells: Vec<Cell>,
	thickness: i32,
	outline: bool,
}
impl Grid {
    pub fn new(rows: usize, cols: usize, size: (u32, u32)) -> Result<Self, String> {
        let mut cells = Vec::with_capacity(rows * cols);
        let width = size.0 as usize / cols;
        let height = size.1 as usize / rows;
        for row in 0..rows {
            for col in 0..cols {
                let cell = Cell::new((col * width) as i32, (row * height) as i32, width as u32, height as u32, Color::RGBA(0, 255, 0, 0xFF)).unwrap();
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