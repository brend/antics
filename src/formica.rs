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
    ReleasePh(u32),
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