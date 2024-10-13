use std::collections::HashMap;
use crate::ant::{Colony, Pheromone};

#[derive(Debug)]
pub struct Cell {
    pub nest: Option<Colony>,
    pub is_obstacle: bool,
    pub food: u32,
    pub pheromone: Option<Pheromone>,
}

impl Cell {
    fn to_ascii(&self) -> char {
        if self.is_obstacle {
            '#'
        } else if self.food > 0 {
            '*'
        } else if self.nest.is_some() {
            'N'
        } else if self.pheromone.is_some() {
            '.'
        } else {
            '_'
        }
    }
}

/// The World is a hexagonal grid of cells addressed by cube coordinates.
#[derive(Debug)]
pub struct World {
    pub cells: HashMap<(i32, i32, i32), Cell>,
}

impl World {
    pub fn new(size: u32) -> Self {
        let mut cells = HashMap::new();
        for x in -(size as i32)..(size as i32) {
            for y in -(size as i32)..(size as i32) {
                let z = -x - y;
                if x.abs() + y.abs() + z.abs() < size as i32 {
                    cells.insert((x, y, z), Cell {
                        nest: None,
                        is_obstacle: false,
                        food: 0,
                        pheromone: None,
                    });
                }
            }
        }
        World { cells }
    }

    pub fn print(&self) {
        let mut rows: HashMap<i32, Vec<String>> = HashMap::new();

        for ((x, y, z), cell) in &self.cells {
            let ascii = cell.to_ascii();
            let row = rows.entry(*y).or_insert_with(Vec::new);
            row.push(format!(" {}", ascii));
        }

        // Print the world row by row
        let mut sorted_keys: Vec<i32> = rows.keys().cloned().collect();
        sorted_keys.sort();
        for key in sorted_keys {
            let row = &rows[&key];
            let offset = " ".repeat((key.abs() * 2) as usize); // Offset to create a hexagonal appearance
            println!("{}{}", offset, row.join(""));
        }
    }
}