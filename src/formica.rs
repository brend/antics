use crate::ant::{Ant, Scent, Pheromone};
use crate::world::Cell;
use crate::grid::Grid;

#[derive(Debug)]
pub enum Address {
    Absolute(u32),
}

#[derive(Debug)]
pub enum Instruction {
    TurnLeft,
    TurnRight,
    Advance,
    Pickup,
    Drop,
    ReleasePh(u8),
    ErasePh,
    CheckFood,
    CheckPh, 
    CheckNest,
    Jmp(Address),
    Jz(Address),
    Jnz(Address),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut labels = std::collections::HashMap::new();
    let mut current_address = 0;

    // First pass: collect labels
    for line in input.lines() {
        let line = line.trim();
        if line.ends_with(':') {
            let label = line.trim_end_matches(':').to_string();
            labels.insert(label, current_address);
        } else {
            current_address += 1;
        }
    }

    // Second pass: parse instructions
    for line in input.lines() {
        let line = line.trim();
        if line.ends_with(':') {
            continue; // Skip labels
        }

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("TURN_L") => instructions.push(Instruction::TurnLeft),
            Some("TURN_R") => instructions.push(Instruction::TurnRight),
            Some("ADVANCE") => instructions.push(Instruction::Advance),
            Some("PICKUP") => instructions.push(Instruction::Pickup),
            Some("DROP") => instructions.push(Instruction::Drop),
            Some("RELEASE_PH") => {
                let scent = parts.next().unwrap().parse().unwrap();
                instructions.push(Instruction::ReleasePh(scent));
            },
            Some("ERASE_PH") => instructions.push(Instruction::ErasePh),
            Some("CHECK_FOOD") => instructions.push(Instruction::CheckFood),
            Some("CHECK_PH") => instructions.push(Instruction::CheckPh),
            Some("CHECK_NEST") => instructions.push(Instruction::CheckNest),
            Some("JMP") => {
                let label = parts.next().unwrap().to_string();
                if let Some(&address) = labels.get(&label) {
                    instructions.push(Instruction::Jmp(Address::Absolute(address as u32)));
                } else {
                    panic!("Undefined label: {}", label);
                }
            },
            Some("JZ") => {
                let label = parts.next().unwrap().to_string();
                if let Some(&address) = labels.get(&label) {
                    instructions.push(Instruction::Jz(Address::Absolute(address as u32)));
                } else {
                    panic!("Undefined label: {}", label);
                }
            },
            Some("JNZ") => {
                let label = parts.next().unwrap().to_string();
                if let Some(&address) = labels.get(&label) {
                    instructions.push(Instruction::Jnz(Address::Absolute(address as u32)));
                } else {
                    panic!("Undefined label: {}", label);
                }
            },
            _ => panic!("Invalid instruction: {}", line),
        }
    }
    instructions
}

#[derive(Debug)]
pub struct AntState {
    pub pc: u32,
    pub flag: u8,
}

impl AntState {
    pub fn new() -> Self {
        AntState { pc: 0, flag: 0 }
    }
}

pub fn run_instruction(program: &[Instruction], grid: &mut Grid<Cell>, ant: &mut Ant) {
    let instruction = &program[ant.state.pc as usize];
    let mut flag: u8 = 0;
    ant.state.pc += 1;
    match instruction {
        Instruction::Advance => {
            let success = ant.move_forward(grid);
            flag = if success { 1 } else { 0 };
        },
        Instruction::TurnLeft => {
            ant.turn_left();
        },
        Instruction::TurnRight => {
            ant.turn_right();
        },
        Instruction::Pickup => {
            let success = ant.pickup(grid);
            flag = if success { 1 } else { 0 };
        },
        Instruction::Drop => {
            let success = ant.drop(grid);
            flag = if success { 1 } else { 0 };
        },
        Instruction::ReleasePh(scent) => {
            grid.get_mut(&ant.coord).unwrap().pheromone = Some(Pheromone { scent: Scent(*scent), colony: ant.colony });
        },
        Instruction::ErasePh => {
            grid.get_mut(&ant.coord).unwrap().pheromone = None;
        },
        Instruction::CheckFood => {
            flag = grid.get(&ant.coord).map(|cell| cell.food).unwrap_or(0);
        },
        Instruction::CheckPh => {
            flag = grid.get(&ant.coord).and_then(|cell| cell.pheromone).map(|p| p.scent.0).unwrap_or(0);
        },
        Instruction::CheckNest => {
            flag = grid.get(&ant.coord).and_then(|cell| cell.nest).map(|n| n.0).unwrap_or(0);
        },
        Instruction::Jmp(Address::Absolute(address)) => {
            ant.state.pc = *address;
        },
        Instruction::Jz(Address::Absolute(address)) => {
            if ant.food == 0 {
                ant.state.pc = *address;
            }
        },
        Instruction::Jnz(Address::Absolute(address)) => {
            if ant.food != 0 {
                ant.state.pc = *address;
            }
        },
    }
    ant.state.flag = flag;
}