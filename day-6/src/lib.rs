pub fn run() {
    let s = include_str!("../input");

    let result = first_marker(s);

    match result {
        Some(i) => println!("{}", i),
        None => eprintln!("Error Parsing"),
    }
}

pub fn is_unique_str(string: &str) -> bool {
    let mut chars = string.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();

    chars.len() == string.len()
}

pub fn first_marker(string: &str) -> Option<usize> {
    let s = String::from(string);

    for (i, _) in string.chars().enumerate() {
        // updated to part 2
        let current_slice = &s[i..i + 14];
        if is_unique_str(current_slice) {
            return Some(i + 14);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_five() {
        // arrange
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let expected = 23;

        // act
        let actual = first_marker(s).unwrap();

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_for_six() {
        // arrange
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        let expected = 23;

        // act
        let actual = first_marker(s).unwrap();

        // assert
        assert_eq!(expected, actual);
    }
}
