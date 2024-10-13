use std::collections::HashMap;
use crate::ant::{Colony, Pheromone, Ant};

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

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Coord { x, y, z }
    }
}

/// The World is a hexagonal grid of cells addressed by cube coordinates.
#[derive(Debug)]
pub struct World {
    pub cells: HashMap<Coord, Cell>,
    pub ants: HashMap<Coord, Ant>,
}

impl World {
    pub fn new(size: u32) -> Self {
        let mut cells = HashMap::new();

        // Iterate over a range of cube coordinates within the specified radius
        for x in -(size as i32)..=(size as i32) {
            for y in -(size as i32)..=(size as i32) {
                let z = -x - y;
                // Only include cells within the hexagonal radius
                if x.abs() <= size as i32 && y.abs() <= size as i32 && z.abs() <= size as i32 {
                    cells.insert(Coord::new(x, y, z), Cell {
                        nest: None,
                        is_obstacle: false,
                        food: 0,
                        pheromone: None,
                    });
                }
            }
        }

        World { 
            cells,
            ants: HashMap::new(),
        }
    }

    fn get_cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        self.cells.get_mut(&coord)
    }

    pub fn add_obstacle(&mut self, coord: Coord) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.is_obstacle = true;
        }
    }

    pub fn add_food(&mut self, coord: Coord, amount: u32) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.food += amount;
        }
    }

    pub fn add_ant(&mut self, coord: Coord, ant: Ant) {
        self.ants.insert(coord, ant);
    }

    pub fn print(&self) {
        let mut rows: HashMap<i32, Vec<String>> = HashMap::new();

        for (coord, cell) in &self.cells {
            let ascii = if self.ants.get(&coord).is_some() { '@' } else { cell.to_ascii() };
            let row = rows.entry(coord.y).or_insert_with(Vec::new);
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