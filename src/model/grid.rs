use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::model::cell::Cell;

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Result<Self, String> {
        let mut cells = Vec::with_capacity(rows * cols);
        let width = 800 / cols;
        let height = 600 / rows;
        for row in 0..rows {
            for col in 0..cols {
                let cell = Cell::new(row as i32, col as i32, width as u32, height as u32, Color::RGB(255, 0, 0)).unwrap();
                cells.push(cell);
            }
        }
        Ok(Grid { cells })
    }

    pub fn draw (&self, canvas: &mut WindowCanvas)  {
        for cell in &self.cells {
            cell.draw(canvas)
        }
    }
}