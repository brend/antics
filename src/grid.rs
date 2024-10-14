use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,   
}

impl Direction {
    pub fn to_cube(&self) -> (i32, i32, i32) {
        match self {
            Direction::North => (-1, 1, 0),
            Direction::NorthEast => (-1, 0, 1),
            Direction::SouthEast => (0, -1, 1),
            Direction::South => (1, -1, 0),
            Direction::SouthWest => (1, 0, -1),
            Direction::NorthWest => (0, 1, -1),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coord {
    pub r: i32,
    pub s: i32,
    pub q: i32,
}

impl Coord {
    pub fn new(r: i32, s: i32, q: i32) -> Self {
        Coord { r, s, q }
    }

    pub fn move_x(&self, dr: i32) -> Self {
        Coord::new(self.r, self.s + dr, self.q - dr)
    }

    pub fn offset_by(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Coord::new(self.r + 1, self.s, self.q - 1),
            Direction::NorthEast => Coord::new(self.r, self.s + 1, self.q - 1),
            Direction::SouthEast => Coord::new(self.r - 1, self.s + 1, self.q),
            Direction::South => Coord::new(self.r - 1, self.s, self.q + 1),
            Direction::SouthWest => Coord::new(self.r, self.s - 1, self.q + 1),
            Direction::NorthWest => Coord::new(self.r + 1, self.s - 1, self.q),
        }
    }
}

pub struct Grid<T> {
    pub cells: HashMap<Coord, T>,
}

impl<T: Default> Grid<T> {
    pub fn new(size: u32) -> Self {
        let size = size as i32;
        let mut cells = HashMap::new();
        for r in -size..=size {
            for s in -size..=size {
                let q = -r - s;
                if r.abs() <= size && s.abs() <= size && q.abs() <= size {
                    cells.insert(Coord::new(r, s, q), T::default());
                }
            }
        }
        Grid { cells }
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        self.cells.insert(coord, value);
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.cells.get(coord)
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut T> {
        self.cells.get_mut(coord)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord, &T)> {
        self.cells.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &Coord> {
        self.cells.keys()
    }

    pub fn get_neighbor(&self, coord: &Coord, dir: &Direction) -> Coord {
        let (dr, ds, dq) = dir.to_cube();
        Coord::new(coord.r + dr, coord.s + ds, coord.q + dq)
    }
}