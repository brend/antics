use crate::formica::AntState;
use crate::grid::{Direction, Coord};
use crate::world::Cell;
use crate::grid::Grid;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Colony(pub u8);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Scent(pub u8);

impl Scent {
    pub fn to_ascii(&self) -> char {
        let c = (self.0 % 26) as u8 + b'a';
        c as char
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pheromone {
    pub scent: Scent,
    pub colony: Colony,
}

impl Pheromone {
    pub fn new(scent: Scent, colony: Colony) -> Self {
        Pheromone { scent, colony }
    }
}

#[derive(Debug)]
pub struct Ant {
    pub colony: Colony,
    pub coord: Coord,
    pub food: u8,
    pub facing: Direction,
    pub state: AntState,
}

impl Ant {
    pub fn new(colony: Colony, coord: Coord) -> Self {
        Ant {
            colony,
            coord,
            food: 0,
            facing: Direction::North,
            state: AntState::new(),
        }
    }

    pub fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::NorthWest,
            Direction::NorthEast => Direction::North,
            Direction::SouthEast => Direction::NorthEast,
            Direction::South => Direction::SouthEast,
            Direction::SouthWest => Direction::South,
            Direction::NorthWest => Direction::SouthWest,
        };
    }

    pub fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        };
    }

    pub fn move_forward(&mut self, grid: &Grid<Cell>) -> bool {
        let new_coord = grid.get_neighbor(&self.coord, &self.facing);
        if grid.get(&new_coord).map(|cell| !cell.is_obstacle).unwrap_or(false) {
            self.coord = new_coord;
            true
        } else {
            false
        }
    }

    pub fn to_ascii(&self) -> char {
        '🐜'
    }

    pub fn pickup(&mut self, grid: &mut Grid<Cell>) -> bool {
        if grid.get_mut(&self.coord).map(|cell| cell.food > 0).unwrap_or(false) {
            self.food += 1;
            grid.get_mut(&self.coord).map(|cell| cell.food -= 1);
            true
        } else {
            false
        }
    }

    pub fn drop(&mut self, grid: &mut Grid<Cell>) -> bool {
        if self.food > 0 {
            self.food -= 1;
            grid.get_mut(&self.coord).map(|cell| cell.food += 1);
            true
        } else {
            false
        }
    }

    pub fn bark(&self) {
        println!("ant@{:?} barked", self.coord);
    }
}