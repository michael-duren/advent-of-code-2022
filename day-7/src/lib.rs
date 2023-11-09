use std::str::Lines;

use regex::Regex;

pub fn run() {
    println!("Hello, world!");
}

pub fn find_small_directories(lines: Lines<'_>) -> usize {
    let file_re = Regex::new(r"\d+(\s|\w|.)+").unwrap();
    let mut dir_content: Vec<Vec<usize>> = vec![];

    for line in lines {
        if file_re.is_match(line) {
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

            // if we were previously in a dir then push to current
            let current_len = dir_content.len() - 1;
            dir_content[current_len].push(file_size);
            continue;
        }

        if line.contains("ls") {
            dir_content.push(vec![]);
        }
    }

    let mut result: usize = 0;

    for dir in dir_content {
        let dir_total = dir.into_iter().sum::<usize>();
        if dir_total < 100000 {
            result += dir_total;
        }
    }

    result
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
