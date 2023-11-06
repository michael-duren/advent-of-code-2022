pub fn run() {
    println!("Hello, world!");
}

pub fn first_marker(string: &str) -> Option<i32> {
    let string = String::from(string);

    for (i, c) in string.chars().enumerate() {
        // if we cannot go past 4 characters ahead break
        if i + 4 > string.len() {
            break;
        }

        // check if the next 4 characters are the same
        for j in 1..4 {
            print!("{}", string.chars().nth(i + j).unwrap());
            if j == 3 {
                return Some(i as i32);
            }

            if c != string.chars().nth(i + j).unwrap() {
                continue;
            }

            if c == string.chars().nth(i + j).unwrap() {
                break;
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_five() {
        // arrange
        let s = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let expected = 5;

        // act
        let actual = first_marker(s).unwrap();

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_for_six() {
        // arrange
        let s = "nppdvjthqldpwncqszvftbrmjlhg";
        let expected = 6;

        // act
        let actual = first_marker(s).unwrap();

        // assert
        assert_eq!(expected, actual);
    }
}
