extern crate drawille; 

use std::thread;
use std::mem;

use self::drawille::{block, braille};

use super::grid::Grid;
use super::grid::FileGridError;

pub struct Game {
    current_grid: Grid,
    new_grid: Grid,
}
impl Game {
    fn tick(&mut self) {
        for y in 0..self.current_grid.height {
            for x in 0..self.current_grid.width {
                let mut neighbours = 0;
                neighbours += (self.current_grid.get((x as i32)-1, (y as i32)-1).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)  , (y as i32)-1).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)+1, (y as i32)-1).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)-1, (y as i32)  ).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)+1, (y as i32)  ).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)-1, (y as i32)+1).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)  , (y as i32)+1).alive == true) as u32;
                neighbours += (self.current_grid.get((x as i32)+1, (y as i32)+1).alive == true) as u32;

                let current_cell = self.current_grid.get(x as i32, y as i32);
                let new_cell = self.new_grid.get_mut(x as i32, y as i32);
                new_cell.alive = match neighbours {
                    2 if current_cell.alive => true,
                    3 => true,
                    _ => false
                }
            }
        }
        mem::swap(&mut self.current_grid, &mut self.new_grid);
    }

    fn run_loop<F>(&mut self, interval: f32, mut draw_closure: F)
        where F : FnMut(&mut Game) {
        let mut generation: u32 = 0;

        print!("\x1B[?25l");  // Hide cursor
        print!("\x1B[2J");  // Clear screen
        loop {
            print!("\x1B[0;0H");  // Reset cursor
            println!("Running Game of Life with {} fps", 1.0/interval);
            println!("Generation: {}", generation);

            draw_closure(self);

            thread::sleep_ms((interval * 1000.0) as u32);
            self.tick();
            generation += 1;
        }
    }

    pub fn run_ansi(&mut self, interval: f32) {
        self.run_loop(interval, |game| {
            game.current_grid.draw_ansi();
        });
    }

    pub fn run_block(&mut self, interval: f32) {
        let mut canvas = block::Canvas::new(self.current_grid.width,
                                              self.current_grid.height);
        self.run_loop(interval, |game| {
            game.current_grid.draw_block(&mut canvas);
            println!("{}", canvas.frame());
        });
    }

    pub fn run_braille(&mut self, interval: f32) {
        let mut canvas = braille::Canvas::new(self.current_grid.width,
                                              self.current_grid.height);
        self.run_loop(interval, |game| {
            game.current_grid.draw_braille(&mut canvas);
            println!("{}", canvas.frame());
        });
    }

    pub fn random_game(width: usize, height: usize) -> Game {
        Game { current_grid: Grid::random_grid(width, height),
               new_grid:     Grid::empty_grid(width, height) }
    }

    pub fn file_game(filename: &str) -> Result<Game, FileGridError>  {
        let current_grid = try!(Grid::file_grid(filename));
        let new_grid = Grid::empty_grid(current_grid.width, current_grid.height);
        Ok(Game { current_grid: current_grid,
               new_grid:     new_grid })
    }
}
