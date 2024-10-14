use antics::world::World;
use antics::grid::Coord;
use antics::ant::{Colony, Ant};
use antics::formica::{parse, Instruction};

fn create_world(program: Vec<Instruction>) -> World {
    let mut world = World::new(9, program);
    let colony1 = Colony(1);
    let nest1 = Coord::new(8, -8, 0);
    for nest_coord in vec![
        Coord::new(8, -8, 0),
        Coord::new(8, -7, -1),
        Coord::new(7, -7, 0),
        Coord::new(7, -8, 1),
    ] {
        world.set_nest(nest_coord, colony1);
    }
    world.add_ant(Ant::new(colony1, nest1));
    world.set_food(Coord::new(6, -6, 0), 1);
    world
}

fn main() {
    let source = std::fs::read_to_string("brains/forward.fmc")
        .expect("Failed to read file");
    let program = parse(&source);
    let mut world = create_world(program);
    loop {
        world.serialize_as_html("ants.html").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        world.update();
    }
}
