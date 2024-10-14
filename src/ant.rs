use crate::formica::{Instruction, Address};
use crate::grid::Direction;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Colony(pub u32);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Scent(u32);

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
pub struct Input {
    pub is_carrying_food: bool,
    pub is_food_on_ground: bool,
    pub is_in_nest: bool,
    pub pheromone: Option<Pheromone>,
}

#[derive(Debug)]
pub enum Action {
    TurnLeft,
    TurnRight,
    MoveForward,
    PickUpFood,
    DropFood,
    ReleasePheromone(Scent),
    ErasePheromone,
}

#[derive(Debug)]
pub struct Ant {
    pub colony: Colony,
    pub food: u32,
    pub facing: Direction,
    pub program_counter: u32,
    pub program: Vec<Instruction>,
}

impl Ant {
    pub fn new(colony: Colony, program: Vec<Instruction>) -> Self {
        Ant {
            colony,
            food: 0,
            facing: Direction::North,
            program_counter: 0,
            program
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

    pub fn to_ascii(&self) -> char {
        '🐜'
    }

    pub fn advance(&mut self) {
        let instruction = &self.program[self.program_counter as usize];
        match instruction {
            Instruction::Advance => {
                self.program_counter += 1;
            },
            Instruction::TurnLeft => {
                self.turn_left();
                self.program_counter += 1;
            },
            Instruction::TurnRight => {
                self.turn_right();
                self.program_counter += 1;
            },
            Instruction::Pickup => {
                self.program_counter += 1;
            },
            Instruction::Drop => {
                self.program_counter += 1;
            },
            Instruction::ReleasePh(scent) => {
                self.program_counter += 1;
            },
            Instruction::ErasePh => {
                self.program_counter += 1;
            },
            Instruction::CheckFood => {
                self.program_counter += 1;
            },
            Instruction::CheckPh => {
                self.program_counter += 1;
            },
            Instruction::CheckNest => {
                self.program_counter += 1;
            },
            Instruction::Jmp(Address::Absolute(address)) => {
                self.program_counter = *address;
            },
            Instruction::Jz(Address::Absolute(address)) => {
                if self.food == 0 {
                    self.program_counter = *address;
                } else {
                    self.program_counter += 1;
                }
            },
            Instruction::Jnz(Address::Absolute(address)) => {
                if self.food != 0 {
                    self.program_counter = *address;
                } else {
                    self.program_counter += 1;
                }
            },
        }
    }
}