use antics::world::{World, Coord};
use antics::ant::Ant;

fn create_world() -> World {
    // create a world
    let mut world = World::new(10);
    world.add_ant(Coord::new(0, 0, 0), Ant {});
    world
}

fn main() {
    let mut world = create_world();
    loop {
        world.display();
        world.update();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
