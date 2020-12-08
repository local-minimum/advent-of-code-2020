use std::fs;

fn load_demo() -> String {
    r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#.to_string()
}

fn load_data() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[derive(Clone)]
struct Instruction {
    instruction: String,
    value: i32,
    visited: bool,
}

impl Instruction {
    fn set_visited(&mut self) {
        self.visited = true;
    }

    fn clear_visit(&mut self) {
        self.visited = false;
    }

    fn mutate(&mut self) {
        match &self.instruction[..] {
            "jmp" => {self.instruction = "nop".to_string()},
            "nop" => {self.instruction = "jmp".to_string()},
            _ => {}
        }
    }
}

fn parse_line(line: String) -> Option<Instruction> {
    match &line[..3] {
        "jmp" | "acc" | "nop" => {            
            match line[4..].parse() {
                Ok(v) => Some(Instruction {
                    instruction: line[..3].to_string(),
                    value: v,
                    visited: false,
                }),
                Err(_) => None,
            }
        },
        _ => None
    }
}

fn execute_code(mut instructions: Vec<Instruction>, verbose: bool) -> (i32, bool) {
    let mut total: i32 = 0;
    let mut operations: usize = 0;
    let mut line: usize = 0;    
    loop {
        let instruction = instructions.get_mut(line).unwrap();
        if verbose {println!("{}: {} {} (total: {}, op {})", line, instruction.instruction, instruction.value, total, operations);}
        if instruction.visited {
            if verbose { println!("Loop found"); }
            return (total, false)
        } else {
            instruction.set_visited();
        }
        match &instruction.instruction[..] {
            "acc" => {
                total += instruction.value;
                line += 1;
            },
            "jmp" => {
                let new_line = line as i32 + instruction.value;
                if new_line >= 0 {
                    line = new_line as usize
                } else {
                    println!("Reached negative line {}", new_line);
                    return (total, false)
                }
            },
            "nop" => { line += 1; },
            _ => {
                println!("Unknown instruction {} on line {}", instruction.instruction, line);
                return (total, false)
            }
        }
        operations += 1;
        if line == instructions.len() {
            if verbose { println!("Terminated"); }
            return (total, true)
        }
    }

}

fn main() {
    // let data = load_demo();
    let data = load_data();
    let instructions: Vec<Instruction> = data
        .lines()
        .map(|v| parse_line(v.to_string()))
        .filter(|v| match v {Some(_) => true, None => false})
        .map(|v| v.unwrap())
        .collect();

    println!("Parsed {} instructions", instructions.len());
    let (part1, part1_term) = execute_code(instructions.clone(), true);
    for idx in 0..instructions.len() {
        let mut instructions_ver = instructions.clone();
        if idx > 0 {
            instructions_ver.get_mut(idx - 1).unwrap().mutate();
        }
        instructions_ver.get_mut(idx).unwrap().mutate();
        let (part2, part2_term) = execute_code(instructions_ver, false);
        println!("Part 2: mutated {}, got value is {} and is terminated {}", idx, part2, part2_term);
        if part2_term {
            break;
        }
    }
    println!("\nPart 1: value is {} and looping: {}", part1, part1_term == false);
}
