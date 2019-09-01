// Game of life rules
// Any cell with 2 || 3 neighbors lives
// Otherwise a cell dies

extern crate termion;

use std::{thread, time};
use std::fmt;
use termion::clear;

#[derive(Debug)]
struct Cell<T> {
    value: T,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct World {
    cells:Vec<Cell<bool>>,
    width: usize,
    height: usize,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let cells_default = vec![false; width * height];
        let x_from_i = |i| i % width;
        let y_from_i = |i| (i - x_from_i(i)) / width;
        let cells = cells_default.iter()
            .enumerate()
            .map(|(i,c)|{
                Cell{
                    value: *c,
                    x: x_from_i(i),
                    y: y_from_i(i),
                }
            })
            .collect();

        World {
            cells,
            width,
            height,
        }
    }

    fn cell_at(&self, x: usize, y: usize) -> &Cell<bool> {
        let i = y * self.width + x;
        &self.cells[i as usize]
    }

    fn set_cell(&mut self, x: usize, y: usize) {
        let i = y * self.width + x;
        self.cells[i] = Cell {
            value: true,
            x:x,
            y:y,
        };
    }

    fn neighbors(&self, cell: &Cell<bool>) -> Vec<&Cell<bool>> {
        let radius = 3i32;
        let c_x = cell.x as i32;
        let c_y = cell.y as i32;
        let width = self.width as i32;
        let height = self.height as i32;
        let x_from_i = |i| i % radius;
        let y_from_i = |i| (i - x_from_i(i)) / radius;
        (0..9)
            .map(|i|(x_from_i(i) - 1, y_from_i(i) - 1)) // Generate neighbors offset grid
            .filter(|&(x,y)| !(x == 0 && y == 0)) // remove the center reference (our cell)
            .map(|(x, y)|(x + c_x, y + c_y)) // Translate grid to the cell
            .filter(|&(x,y)| x >= 0 && y >= 0 && x < width && y < height) // remove neighbors that violate bounds
            .map(|(x,y)| self.cell_at(x as usize, y as usize)) // Get cells
            .collect()
    }

    fn alive_neighbors(&self, cell: &Cell<bool>) -> Vec<&Cell<bool>> {
        self.neighbors(cell).into_iter().filter(|n| n.value).collect()
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cells.iter().for_each(|c|{
            let count = if c.value { self.neighbors(c).len() } else { 0 };
            let string = if c.value { "X" } else { " " };
            let p = string;
            if c.x == self.width - 1 {
                 write!(f, "{}\n", p).expect("");
            }
            else {
                write!(f, "{}", p).expect("");
            }
        });

        write!(f, "")
    }
}

fn main() {
    let mut world = World::new(40, 20);

    //Glider
    world.set_cell(2,1);
    world.set_cell(3,2);
    world.set_cell(1,3);
    world.set_cell(2,3);
    world.set_cell(3,3);

    let rule_check = |alive, neighbors| {
        if alive {
            neighbors == 2 || neighbors == 3
        } else {
            neighbors == 3
        }
    };

    for _ in 0..80 {
        println!("{}", clear::All);
        println!("{}", world);

        world = World {
            cells: world.cells.iter()
                    .map(|c| Cell {
                        value: rule_check(c.value, world.alive_neighbors(c).len()),
                        ..*c
                    })
                    .collect(),
            ..world
        };

        thread::sleep(time::Duration::from_millis(10));
    }
}
