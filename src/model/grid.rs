use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::model::cell::Cell;


pub struct Grid {
    cells: Vec<Cell>,
	cell_size: u32,
	thickness: i32,
	outline: bool,
}
impl Grid {
    pub fn new(start: Point, end: Point, cell_size: u32) -> Result<Self, String> {
		let rows = (end.y - start.y) / cell_size  as i32;
		let cols = (end.x - start.x) / cell_size as i32;
        let mut cells = Vec::with_capacity(rows as usize * cols as usize);
		let t1 = 0 - start.x as i32;
		let t2 = start.x + cols * cell_size as i32 - 3840;
		let fix_margin = (t1 - t2) / 2;
		println!("{}", fix_margin);
        for row in 0..rows {
            for col in 0..cols {
				let mut cell = Cell::new((start.x + fix_margin + col * cell_size as i32), (start.y + row * cell_size as i32) , Color::RGBA(133, 24, 56, 0xFF)).unwrap();
				cells.push(cell);
            }
        }
		let thickness = 0;
		let outline = false;
        Ok(Grid { cells, cell_size, thickness, outline })
    }

    pub fn draw (&self, canvas: &mut WindowCanvas)  {
        for cell in &self.cells {
            cell.draw(canvas, self.cell_size, self.thickness, self.outline)
        }
	}


	pub fn set_grid_cell_size(&self, cell_size: u32) -> Result<(), String> {
		for cell in &self.cells {
			// cell.set_cellsize(cell_size);
		}
		Ok (())
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