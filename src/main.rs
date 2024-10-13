use antics::world::{World, Coord};
use antics::ant::Ant;

fn main() {
    // create a world
    let mut world = World::new(10);
    // add 7 obstacles at random locations
    for _ in 0..7 {
        let x = rand::random::<i32>() % 10;
        let y = rand::random::<i32>() % 10;
        let z = -x - y;
        world.add_obstacle(Coord::new(x, y, z));
    }
    world.add_food(Coord::new(0, 0, 0), 3);
    // add a couple of ants at random locations
    for _ in 0..2 {
        let x = rand::random::<i32>() % 10;
        let y = rand::random::<i32>() % 10;
        let z = -x - y;
        world.add_ant(Coord::new(x, y, z), Ant {});
    }
    // print the world
    world.print();
}
