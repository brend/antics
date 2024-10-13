#[derive(Debug)]
pub struct Colony(u32);

#[derive(Debug)]
pub struct Scent(u32);

#[derive(Debug)]
pub struct Pheromone {
    pub scent: Scent,
    pub colony: Colony,
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