use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::model::cell::Cell;

const BLACK_OUTLINE: Color = Color::RGBA(0, 0, 0, 0xFF);

pub struct Grid {
    cells: Vec<Cell>,
	cell_size: i32,
	thickness: u32,
    margin: u32,
	outline: bool,
    offset: f32,
}
impl Grid {
    pub fn new(cells: Vec<Cell>, margin: u32, cell_size: i32, thickness: u32, offset: f32, outline: bool) -> Result<Self, String> {
        Ok(Grid {cells, cell_size, thickness, margin, offset, outline})
    }

    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    pub fn size(&self) -> i32 {
    self.cell_size
}

    pub fn thickness(&self) -> u32 {
        self.thickness
    }

    pub fn outline(&self) -> bool {
        self.outline
    }

    pub fn margin(&self) -> u32 {
        self.margin
    }

    pub fn offset(&self) -> f32 {
        self.offset
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
		self.thickness = thickness as u32;
	}
}

pub struct GridBuilder {
    margin: u32,
    width: u32,
    height: u32,
    cell_size: i32,
    outline: bool,
    outline_thickness: u32,
    offset: f32,
}

impl GridBuilder {
    pub fn new (width: u32, height: u32) -> Self{
        GridBuilder {
            margin: 10,
            cell_size: 20,
            outline: true,
            outline_thickness: 1,
            offset: 0.0,
            width,
            height,
        }
    }

    pub fn margin (mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    pub fn outline_thickness(mut self, thickness: u32) -> Self {
        self.outline_thickness = thickness;
        self
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    pub fn build(mut self) -> Result<Grid, String> {
        let rows = (self.height - 2 * self.margin) as i32 / self.cell_size;
        let cols = (self.width - 2 * self.margin) as i32/ self.cell_size;
        let mut cells = Vec::with_capacity((rows * cols) as usize);
        for row in 0..rows {
            for col in 0..cols {
                let mut cell = Cell::new(col as i32,row as i32, Color::RGBA(133, 122, 177, 0xFF)).unwrap();
                cells.push(cell);
            }
        }
       let grid = Grid::new(cells, self.margin, self.cell_size as i32, self.outline_thickness, self.offset, self.outline).unwrap();
        Ok(grid)
    }
}