#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,   
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