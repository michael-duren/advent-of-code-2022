use std::{collections::HashSet, process, str::Lines};

use regex::Regex;

pub fn run() {
    let commands = include_str!("../input").lines();

    let result = find_small_directories(commands);

    println!("Result is: {:?}", result);
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<usize>,
    directories: Vec<String>,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            name,
            files: vec![],
            directories: vec![],
        }
    }
}

pub fn find_small_directories(lines: Lines<'_>) -> usize {
    let file_re = Regex::new(r"\d+(\s|\w|.)+").unwrap();
    let cd_re = Regex::new(r"^\$\scd\s+.+").unwrap();
    let mut dirs: Vec<Directory> = vec![];

    // get dirs contents
    for (i, line) in lines.enumerate() {
        if cd_re.is_match(line) && !line.contains("..") {
            // create new dir
            let dir_name: &str = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .get(2)
                .unwrap_or_else(|| {
                    eprintln!("Error parsing line: {}", line);
                    process::exit(1);
                });

            let directory = Directory::new(String::from(dir_name));
            dirs.push(directory);
        } else if file_re.is_match(line) {
            let mut str_num = String::from("");

            for c in line.chars() {
                if c.is_numeric() {
                    str_num.push(c);
                } else {
                    break;
                }
            }

            // parse file size
            let file_size = str_num.parse::<usize>().unwrap();

            // push into last created dir
            if let Some(last_dir) = dirs.last_mut() {
                last_dir.files.push(file_size);
            } else {
                eprintln!(
                    "Error finding last dir line: {}, line number: {}",
                    line,
                    (i + 1)
                );
                process::exit(1);
            }
            continue;
        } else if line.contains("dir") {
            let dir_name: &str = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or_else(|| {
                    eprintln!("Error parsing line: {}. Line number: {}", line, (i + 1));
                    process::exit(1);
                });

            if let Some(last_dir) = dirs.last_mut() {
                last_dir.directories.push(String::from(dir_name));
            } else {
                eprintln!(
                    "Error finding last dir line: {}, line number: {}",
                    line,
                    (i + 1)
                );
                process::exit(1);
            }
        }
    }

    let mut result: usize = 0;

    for dir in &dirs {
        println!("Dir: {:?}", dir);
        let mut visited: HashSet<String> = HashSet::new();
        let dir_total = sum_dir(dir, &dirs, 0, &mut visited);
        if dir_total <= 100000 {
            result += dir_total;
        }
    }

    result
}

fn sum_dir(
    dir: &Directory,
    dir_list: &[Directory],
    mut total: usize,
    visited: &mut HashSet<String>,
) -> usize {
    if visited.contains(&dir.name) {
        return total;
    }

    visited.insert(dir.name.clone());

    total += dir.files.iter().sum::<usize>();

    for d in &dir.directories {
        if let Some(next_dir) = dir_list.iter().find(|dr| &dr.name == d) {
            total = sum_dir(next_dir, dir_list, total, visited);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        // arrange
        let test_input = include_str!("../test-input").lines();
        let expected = 95437;

        // act
        let actual = find_small_directories(test_input);

        // assert
        assert_eq!(
            expected, actual,
            "Expected: {} should be equal to actual: {}",
            expected, actual
        );
    }
}
