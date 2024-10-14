use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::grid::{Coord, Direction};
use crate::ant::{Colony, Pheromone, Ant, Input, Action};

#[derive(Debug)]
pub struct Cell {
    pub nest: Option<Colony>,
    pub is_obstacle: bool,
    pub food: u32,
    pub pheromone: Option<Pheromone>,
}

impl Cell {
    fn to_ascii(&self) -> char {
        if self.is_obstacle {
            '#'
        } else if self.food > 0 {
            '*'
        } else if self.nest.is_some() {
            'N'
        } else if let Some(p) = self.pheromone {
            p.scent.to_ascii()
        } else {
            '_'
        }
    }
}

/// The World is a hexagonal grid of cells addressed by cube coordinates.
#[derive(Debug)]
pub struct World {
    pub size: u32,
    pub cells: HashMap<Coord, Cell>,
    pub ants: HashMap<Coord, Ant>,
}

impl World {
    pub fn new(size: u32) -> Self {
        let size = size as i32;
        let mut cells = HashMap::new();

        // Iterate over a range of cube coordinates within the specified radius
        for x in -size..=size {
            for y in -size..=size {
                let z = -x - y;
                // Only include cells within the hexagonal radius
                if x.abs() <= size && y.abs() <= size && z.abs() <= size {
                    cells.insert(Coord::new(x, y, z), Cell {
                        nest: None,
                        is_obstacle: false,
                        food: 0,
                        pheromone: None,
                    });
                }
            }
        }

        World { 
            size: size as u32,
            cells,
            ants: HashMap::new(),
        }
    }

    fn get_cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        self.cells.get_mut(&coord)
    }

    pub fn add_obstacle(&mut self, coord: Coord) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.is_obstacle = true;
        }
    }

    pub fn add_food(&mut self, coord: Coord, amount: u32) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.food += amount;
        }
    }

    pub fn add_ant(&mut self, coord: Coord, ant: Ant) {
        self.ants.insert(coord, ant);
    }
    
    pub fn display(&self) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        // Determine grid bounds
        for coord in self.cells.keys() {
            if coord.q < min_x { min_x = coord.q; }
            if coord.q > max_x { max_x = coord.q; }
            if coord.r < min_y { min_y = coord.r; }
            if coord.r > max_y { max_y = coord.r; }
        }

        // Print the grid in a hexagonal format
        for y in (min_y..=max_y).rev() {
            print!("{}", " ".repeat((y - min_y) as usize));  // Offset for hex alignment
            for x in min_x..=max_x {
                let z = -x - y;
                let coord = Coord::new(x, y, z);
                let icon = if let Some(ant) = self.ants.get(&coord) { ant.to_ascii() } else if let Some(value) = self.cells.get(&coord) { value.to_ascii() } else { ' ' };
                print!(" {icon} ");
            }
            println!();
        }
    }
    
    pub fn update(&mut self) {
        // let mut actions: HashMap<Coord, Action> = HashMap::new();
        // let mut moves = vec![];

        // // collect actions
        // for (coord, ant) in &self.ants {
        //     let cell = self.cells.get(coord).unwrap();
        //     let input = Input {
        //         is_carrying_food: ant.food > 0,
        //         is_food_on_ground: cell.food > 0,
        //         is_in_nest: if let Some(nest) = cell.nest { nest == ant.colony } else { false },
        //         pheromone: cell.pheromone,
        //     };
        //     actions.insert(*coord, ant.decide(&input));
        // }

        // // execute actions
        // for (coord, action) in &actions {
        //     match action {
        //         Action::MoveForward => {
        //             let new_coord = match self.ants.get(coord) {
        //                 Some(ant) => {
        //                     let (r, s, q) = (coord.r, coord.s, coord.q);
        //                     match ant.facing {
        //                         Direction::North => Coord::new(r + 1, s, q - 1),
        //                         Direction::NorthEast => Coord::new(r, s + 1, q - 1),
        //                         Direction::SouthEast => Coord::new(r - 1, s + 1, q),
        //                         Direction::South => Coord::new(r - 1, s, q + 1),
        //                         Direction::SouthWest => Coord::new(r, s - 1, q + 1),
        //                         Direction::NorthWest => Coord::new(r + 1, s - 1, q),
        //                     }
        //                 },
        //                 None => *coord,
        //             };
        //             moves.push((coord, new_coord));
        //         },
        //         Action::TurnLeft => {
        //             if let Some(ant) = self.ants.get_mut(coord) {
        //                 ant.turn_left();
        //             }
        //         },
        //         Action::TurnRight => {
        //             if let Some(ant) = self.ants.get_mut(coord) {
        //                 ant.turn_right();
        //             }
        //         },
        //         Action::PickUpFood => {
        //             if let Some(cell) = self.get_cell_mut(*coord) {
        //                 if cell.food > 0 {
        //                     cell.food -= 1;
        //                     if let Some(ant) = self.ants.get_mut(coord) {
        //                         ant.food += 1;
        //                     }
        //                 }
        //             }
        //         },
        //         Action::DropFood => {
        //             if let Some(cell) = self.get_cell_mut(*coord) {
        //                 cell.food += 1;
        //                 if let Some(ant) = self.ants.get_mut(coord) {
        //                     ant.food -= 1;
        //                 }
        //             }
        //         },
        //         Action::ReleasePheromone(scent) => {
        //             let colony = self.ants.get(coord).unwrap().colony;
        //             if let Some(cell) = self.get_cell_mut(*coord) {
        //                 cell.pheromone = Some(Pheromone::new(*scent, colony));
        //             }
        //         },
        //         Action::ErasePheromone => {
        //             if let Some(cell) = self.get_cell_mut(*coord) {
        //                 cell.pheromone = None;
        //             }
        //         },
        //     }
        // }

        // // execute move actions
        // for (old_coord, new_coord) in moves {
        //     if let Some(ant) = self.ants.remove(&old_coord) {
        //         self.ants.insert(new_coord, ant);
        //     }
        // }
    }

    pub fn serialize_as_html(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;

        // HTML and SVG header
        writeln!(file, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ant World</title>
    <style>
        .hex {{
            stroke: #333;
            stroke-width: 1;
            fill: #eee;
        }}
        .nest {{ fill: #ffcc00; }}
        .obstacle {{ fill: #333; }}
        .food {{ fill: #ff4444; }}
        .pheromone {{ fill: #99ccff; }}
    </style>
</head>
<body>
    <svg width="100%" height="100%" viewBox="-100 -100 800 800">
"#)?;

        let hex_radius = 20.0; // Radius of each hexagon
        let hex_width = hex_radius * 2.0;
        let hex_height = 3f64.sqrt() * hex_radius;

        for (coord, cell) in &self.cells {
            let (x, y) = (coord.q, coord.r);
            let center_x = x as f64 * hex_width * 0.75; // Shift x by 75% of hex width
            let center_y = y as f64 * hex_height + (x as f64 * hex_height / 2.0); // Offset y by x-based offset

            // Define points for the hexagon shape
            let points: Vec<String> = (0..6)
                .map(|i| {
                    let angle = std::f64::consts::PI / 3.0 * i as f64;
                    let px = center_x + hex_radius * angle.cos();
                    let py = center_y + hex_radius * angle.sin();
                    format!("{},{}", px, py)
                })
                .collect();

            let mut class = "hex";

            // Determine the class based on cell properties
            if cell.is_obstacle {
                class = "obstacle";
            } else if cell.food > 0 {
                class = "food";
            } else if cell.nest.is_some() {
                class = "nest";
            } else if cell.pheromone.is_some() {
                class = "pheromone";
            }

            writeln!(
                file,
                r#"<polygon points="{}" class="{}" />"#,
                points.join(" "),
                class
            )?;
        }

        // Close SVG and HTML tags
        writeln!(file, r#"
    </svg>
</body>
</html>
"#)?;

        Ok(())
    }
}