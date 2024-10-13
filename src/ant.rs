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
pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug)]
pub struct Ant {
    pub colony: Colony,
    pub food: u32,
    pub facing: Direction,
}

impl Ant {
    pub fn new(colony: Colony) -> Self {
        Ant {
            colony,
            food: 0,
            facing: Direction::North,
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

    pub fn decide(&self, input: &Input) -> Action {
        let r = rand::random::<f32>();

        if r < 0.3 {
            Action::ReleasePheromone(Scent(rand::random()))
        } else {
            Action::MoveForward
        }
    }

    pub fn to_ascii(&self) -> char {
        match self.facing {
            Direction::North => '^',
            Direction::NorthEast => '/',
            Direction::SouthEast => 'v',
            Direction::South => 'v',
            Direction::SouthWest => '\\',
            Direction::NorthWest => '^',
        }
    }
}