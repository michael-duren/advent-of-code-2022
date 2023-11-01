use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    total: u32,
    from: u32,
    to: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl From<std::num::ParseIntError> for ParseInstructionError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseInstructionError
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instruction_nums = vec![];

        let digit_re = Regex::new(r"[0-9]+").unwrap();

        s.split(" ").for_each(|x| {
            if digit_re.is_match(x) {
                instruction_nums.push(x);
            }
        });

        if instruction_nums.len() != 3 {
            println!("Error parsing instruction: {:?}", instruction_nums);
            return Err(ParseInstructionError);
        }

        let instruction: Instruction = Instruction {
            total: instruction_nums[0].parse::<u32>()?,
            from: instruction_nums[1].parse::<u32>()?,
            to: instruction_nums[2].parse::<u32>()?,
        };

        return Ok(instruction);
    }
}

fn parse_crates(line: &str) -> Vec<char> {
    let mut row = vec![];

    for c in line.chars() {
        if c.is_alphabetic() {
            row.push(c);
        }
    }

    return row;
}

fn main() {
    let lines = include_str!("../test-input").lines();
    let mut rows = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    let create_re = Regex::new(r"[A-Z]").unwrap();
    let move_re = Regex::new(r"move").unwrap();

    for line in lines {
        if create_re.is_match(line) {
            let row = parse_crates(line);
            println!("{:?}", row);
            rows.push(row);
            continue;
        }
        if move_re.is_match(line) {
            let instruction = Instruction::from_str(line).unwrap();
            instructions.push(instruction);
        }
    }

    for row in rows {
        println!("{:?}", row);
    }
    for instruction in instructions {
        println!("{:?}", instruction);
    }
}
