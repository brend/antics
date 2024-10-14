use antics::world::World;
use antics::grid::Coord;
use antics::ant::{Colony, Ant};
use antics::formica::{parse, Instruction};

fn create_world(program: Vec<Instruction>) -> World {
    let mut world = World::new(3, program);
    world.add_food(Coord::new(1, 0, -1), 1);
    world.add_ant(Ant::new(Colony(0), Coord::new(0, 0, 0)));
    world
}

fn main() {
    let program = parse("loop:\n\
                         ADVANCE\n\
                         TURN_L\n\
                         JMP loop\n");
    let mut world = create_world(program);
    loop {
        world.serialize_as_html("ants.html").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        world.update();
    }
}
