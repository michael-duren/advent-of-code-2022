use regex::Regex;
use std::{error::Error, process, str::FromStr};

pub fn run(lines: core::str::Lines<'_>) -> Result<(), Box<dyn Error>> {
    let mut rows = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    let create_re = Regex::new(r"[A-Z]").unwrap();
    let move_re = Regex::new(r"move").unwrap();

    // create rows and instructions
    for line in lines {
        if create_re.is_match(line) {
            let row = parse_crates(line);
            rows.push(row);
            continue;
        }
        if move_re.is_match(line) {
            let instruction = Instruction::from_str(line).unwrap();
            instructions.push(instruction);
        }
    }

    println!("Rows before rotation: {:?}", rows);

    // rotate rows
    let mut rotated_rows = rotate_rows(rows);

    println!("{:?}", rotated_rows);

    for instruction in instructions {
        println!("{:?}", rotated_rows);
        let result = move_crates(&mut rotated_rows, &instruction);

        match result {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("ISSUE RUNNING INSTRUCTIONS: {}", e);
                process::exit(1);
            }
        }
    }

    return Ok(());
}

fn move_crates(rows: &mut Vec<Vec<char>>, instruction: &Instruction) -> Result<(), &'static str> {
    for _ in 0..instruction.total {

        // if let Some(item) = rows[instruction.from].pop() {
        //     println!("Item is being moved. Item: {}", item);
        //     rows[instruction.to].push(item);
        // } else {
        //     return Err("Error moving crates");
        // }
    }

    return Ok(());
}

fn rotate_rows(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let longest_row: usize = rows.iter().max_by_key(|x| x.len()).unwrap().len(); // find the length of the longest row

    let mut transposed_rows: Vec<Vec<char>> = vec![Vec::new(); rows.len()];
    println!("Transposed Rows Empty: {:?}", transposed_rows);

    for row in rows.iter().rev() {
        for (i, &c) in row.iter().enumerate() {
            transposed_rows[i].push(c);
        }
        // for (i, c) in row.iter().enumerate() {
        //     transposed_rows[i].push(*c);
        // }

        // for i in row.len()..longest_row {
        //     transposed_rows[i].push(' ');
        // }
    }

    return transposed_rows;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    total: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseInstructionError;

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
            total: instruction_nums[0].parse()?,
            from: instruction_nums[1].parse()?,
            to: instruction_nums[2].parse()?,
        };

        return Ok(instruction);
    }
}

pub fn parse_crates(line: &str) -> Vec<char> {
    let mut row = vec![];

    for (i, c) in line.chars().enumerate() {
        if c.is_whitespace() && i % 4 == 0 {
            row.push(' ');
        } else if c.is_alphabetic() {
            row.push(c);
        }
    }

    return row;
}
