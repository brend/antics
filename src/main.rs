use antics::world::{World, Coord};
use antics::ant::{Colony, Ant};

fn create_world() -> World {
    let mut world = World::new(10);
    world.add_ant(Coord::new(0, 0, 0), Ant::new(Colony(0)));
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
