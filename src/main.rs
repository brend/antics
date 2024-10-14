use antics::world::World;
use antics::grid::Coord;
use antics::ant::{Colony, Ant};
use antics::formica::{parse, Instruction};

fn create_world(program: Vec<Instruction>) -> World {
    let mut world = World::new(3);
    world.add_ant(Coord::new(0, 0, 0), Ant::new(Colony(0), program));
    world
}

fn main() {
    // let mut world = create_world();
    // loop {
    //     world.display();
    //     world.update();
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    // }
    let program = parse("loop:\n\
                         ADVANCE\n\
                         TURN_L\n\
                         JMP loop\n");
    let mut world = create_world(program);
    world.serialize_as_html("ants.html");
    loop {
        world.display();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        world.update();
    }
}
