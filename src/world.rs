use std::fs::File;
use std::io::Write;
use crate::grid::{Grid, Coord, Direction};
use crate::ant::{Colony, Pheromone, Scent, Ant};
use crate::formica::{Instruction, run_instruction};

#[derive(Default)]
pub struct Cell {
    pub nest: Option<Colony>,
    pub is_obstacle: bool,
    pub food: u8,
    pub pheromone: Option<Pheromone>,
}

/// The World is a hexagonal grid of cells addressed by cube coordinates.
pub struct World {
    pub size: u32,
    pub grid: Grid<Cell>,
    pub ants: Vec<Ant>,
    pub program: Vec<Instruction>,
}

impl World {
    pub fn new(size: u32, program: Vec<Instruction>) -> Self {
        World { 
            size: size as u32,
            grid: Grid::new(size),
            ants: vec![],
            program,
        }
    }

    fn get_cell(&self, coord: &Coord) -> Option<&Cell> {
        self.grid.cells.get(coord)
    }

    fn get_cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        self.grid.get_mut(&coord)
    }

    pub fn add_obstacle(&mut self, coord: Coord) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.is_obstacle = true;
        }
    }

    pub fn add_food(&mut self, coord: Coord, amount: u8) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.food += amount;
        }
    }

    pub fn remove_food(&mut self, coord: Coord, amount: u8) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.food = cell.food.saturating_sub(amount);
        }
    }

    pub fn get_food_amount(&self, coord: Coord) -> u8 {
        if let Some(cell) = self.get_cell(&coord) {
            cell.food
        } else {
            0
        }
    }

    pub fn is_obstacle(&self, coord: &Coord) -> bool {
        if let Some(cell) = self.get_cell(coord) {
            cell.is_obstacle
        } else {
            false
        }
    }

    pub fn add_ant(&mut self, ant: Ant) {
        self.ants.push(ant);
    }

    pub fn get_ant(&self, coord: &Coord) -> Option<&Ant> {
        self.ants.iter().find(|ant| ant.coord == *coord)
    }

    pub fn get_pheromone(&self, coord: &Coord, colony: Colony) -> Option<Pheromone> {
        if let Some(cell) = self.get_cell(coord) {
            cell.pheromone.filter(|p| p.colony == colony)
        } else {
            None
        }
    }

    pub fn add_pheromone(&mut self, coord: Coord, scent: Scent, colony: Colony) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.pheromone = Some(Pheromone { scent, colony });
        }
    }

    pub fn erase_pheromone(&mut self, coord: Coord) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.pheromone = None;
        }
    }

    pub fn get_nest(&self, coord: &Coord) -> Option<Colony> {
        if let Some(cell) = self.get_cell(coord) {
            cell.nest
        } else {
            None
        }
    }
    
    pub fn get_neighbor(&self, coord: &Coord, direction: &Direction) -> Coord {
        let (x, y, z) = (coord.q, coord.r, coord.s);
        let (dx, dy, dz) = direction.to_cube();
        Coord::new(x + dx, y + dy, z + dz)
    }
    
    pub fn update(&mut self) {
        for ant in &mut self.ants {
            run_instruction(&self.program, &mut self.grid, ant);
        }
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
            text {{
                font-size: 20px;
                text-anchor: middle;
                dominant-baseline: central;
            }}
        </style>
    </head>
    <body>
        <svg width="100%" height="100%" viewBox="-140 -140 900 900">
    "#)?;
    
        let hex_radius = 20.0; // Radius of each hexagon
        let hex_width = hex_radius * 2.0;
        let hex_height = 3f64.sqrt() * hex_radius;
    
        for (coord, cell) in self.grid.iter() {
            let (x, y) = (coord.q, coord.r);
            let center_x = x as f64 * hex_width * 0.75;
            let center_y = y as f64 * hex_height + (x as f64 * hex_height / 2.0);
    
            // Define points for the hexagon shape
            let points: Vec<String> = (0..6)
                .map(|i| {
                    let angle = std::f64::consts::PI / 3.0 * i as f64;
                    let px = center_x + hex_radius * angle.cos();
                    let py = center_y + hex_radius * angle.sin();
                    format!("{},{}", px, py)
                })
                .collect();
    
            writeln!(
                file,
                r#"<polygon points="{}" class="hex" />"#,
                points.join(" ")
            )?;
    
            // Determine the emoji based on cell properties
            let emoji = if cell.is_obstacle {
                "🪨"  // Rock for obstacle
            } else if cell.food > 0 {
                "🍔"  // Burger for food
            } else if cell.nest.is_some() {
                "🏠"  // House for nest
            } else if cell.pheromone.is_some() {
                "☁️" // Cloud for pheromone
            } else {
                ""
            };
    
            if !emoji.is_empty() {
                // Place the emoji at the hexagon center
                writeln!(
                    file,
                    r#"<text x="{}" y="{}">{}</text>"#,
                    center_x, center_y, emoji
                )?;
            }

            // Display the ant at the hexagon center
            if let Some(ant) = self.get_ant(&coord) {
                let ant_emoji = ant.to_ascii();
                // Rotate the ant emoji based on the direction it's facing
                let angle = match ant.facing {
                    Direction::North => 0.0,
                    Direction::NorthEast => 60.0,
                    Direction::SouthEast => 120.0,
                    Direction::South => 180.0,
                    Direction::SouthWest => 240.0,
                    Direction::NorthWest => 300.0,
                };
                // Place the ant emoji at the hexagon center
                writeln!(
                    file,
                    r#"<text x="{}" y="{}" transform="rotate({} {} {})" >{}</text>"#,
                    center_x, center_y, angle, center_x, center_y, ant_emoji
                )?;
            }
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