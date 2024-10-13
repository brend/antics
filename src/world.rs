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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coord {
    r: i32,
    s: i32,
    q: i32,
}

impl Coord {
    pub fn new(r: i32, s: i32, q: i32) -> Self {
        Coord { r, s, q }
    }

    pub fn move_x(&self, dr: i32) -> Self {
        Coord::new(self.r, self.s + dr, self.q - dr)
    }
}

/// The World is a hexagonal grid of cells addressed by cube coordinates.
#[derive(Debug)]
pub struct World {
    pub size: u32,
    pub cells: HashMap<Coord, Cell>,
    pub ants: HashMap<Coord, Ant>,
}

impl World {
    pub fn new(size: u32) -> Self {
        let size = size as i32;
        let mut cells = HashMap::new();

        // Iterate over a range of cube coordinates within the specified radius
        for x in -size..=size {
            for y in -size..=size {
                let z = -x - y;
                // Only include cells within the hexagonal radius
                if x.abs() <= size && y.abs() <= size && z.abs() <= size {
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
            size: size as u32,
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
        let size = self.size as i32;
        // Determine the range of coordinates to display, assuming the grid is centered
        for r in -size..=size {
            // Add padding at the start of each row to shift columns correctly
            let padding = (size / 2 - r).abs() as usize;
            print!("{}", "ö".repeat(padding));

            for q in -size..=size {
                let s = -r - q;
                let coord = Coord { r, s, q };

                if let Some(value) = self.cells.get(&coord) {
                    print!("{} ", value.to_ascii());
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
    
    pub fn display(&self) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        // Determine grid bounds
        for coord in self.cells.keys() {
            if coord.q < min_x { min_x = coord.q; }
            if coord.q > max_x { max_x = coord.q; }
            if coord.r < min_y { min_y = coord.r; }
            if coord.r > max_y { max_y = coord.r; }
        }

        // Print the grid in a hexagonal format
        for y in (min_y..=max_y).rev() {
            print!("{}", " ".repeat((y - min_y) as usize));  // Offset for hex alignment
            for x in min_x..=max_x {
                let z = -x - y;
                let coord = Coord::new(x, y, z);
                let icon = if self.ants.contains_key(&coord) { '@' } else if let Some(value) = self.cells.get(&coord) { value.to_ascii() } else { ' ' };
                print!(" {icon} ");
            }
            println!();
        }
    }

    fn random_ant_moves(&self) -> Vec<(Coord, Coord)> {
        let mut moves = Vec::new();

        for (coord, ant) in &self.ants {
            let new_coord = coord.move_x(1);
            moves.push((*coord, new_coord));
        }

        moves
    }

    pub fn update(&mut self) {
        // For now, just move the ants randomly
        let moves = self.random_ant_moves();

        for (old_coord, new_coord) in moves {
            if let Some(ant) = self.ants.remove(&old_coord) {
                self.ants.insert(new_coord, ant);
            }
        }
    }
}