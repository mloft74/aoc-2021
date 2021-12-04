use std::fs;

fn main() {
    let file_name = String::from("input.txt");
    let contents = fs::read_to_string(&file_name).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let instructions = process_input(&lines);

    println!("part 1: {:?}", part_1(&instructions));
    println!("part 2: {}", part_2(&instructions));
}

fn process_input(lines: &Vec<&str>) -> Vec<Instruction<i32>> {
    lines.into_iter()
        .map(|e| {
            let split: Vec<&str> = e.split(' ').collect();
            let name = split[0];
            let value = split[1];
            Instruction::new(InstructionName::from(name), value.parse().unwrap())
        })
        .collect()
}

fn part_1(instructions: &Vec<Instruction<i32>>) -> i32 {
    let mut position = Position::new(0, 0);
    for instruction in instructions {
        match instruction.instruction_name {
            InstructionName::Forward => position.horizontal += instruction.value,
            InstructionName::Up => position.depth -= instruction.value,
            InstructionName::Down => position.depth += instruction.value,
        }
    }

    position.horizontal * position.depth
}

fn part_2(instructions: &Vec<Instruction<i32>>) -> i32 {
    let mut position = Position::new(0, 0);
    let mut aim = 0;
    for instruction in instructions {
        match instruction.instruction_name {
            InstructionName::Forward => {
                position.horizontal += instruction.value;
                position.depth += aim * instruction.value;
            },
            InstructionName::Up => aim -= instruction.value,
            InstructionName::Down => aim += instruction.value,
        }
    }

    position.horizontal * position.depth
}

enum InstructionName {
    Forward,
    Up,
    Down,
}

impl InstructionName {
    fn from(val: &str) -> InstructionName {
        match val.to_lowercase().as_str() {
            "forward" => InstructionName::Forward,
            "up" => InstructionName::Up,
            "down" => InstructionName::Down,
            _ => panic!("cannot convert '{}' to InstructionName", val),
        }
    }
}

struct Instruction<T> {
    instruction_name: InstructionName,
    value: T,
}

impl<T> Instruction<T> {
    fn new(name: InstructionName, value: T) -> Instruction<T> {
        Instruction {
            instruction_name: name,
            value: value,
        }
    }
}

struct Position<T> {
    horizontal: T,
    depth: T,
}

impl<T> Position<T> {
    fn new(horizontal: T, vertical: T) -> Position<T> {
        Position {
            horizontal: horizontal,
            depth: vertical,
        }
    }
}
