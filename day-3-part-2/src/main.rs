use std::{collections::HashMap, error::Error};

fn get_point_dic() -> HashMap<char, i32> {
    let mut point_dict: HashMap<char, i32> = HashMap::new();

    for (index, char) in ('a'..='z').enumerate() {
        point_dict.insert(char, index as i32 + 1);
    }

    for (index, char) in ('A'..='Z').enumerate() {
        point_dict.insert(char, index as i32 + 27);
    }

    return point_dict;
}

// find the unique letter in three different lines
fn find_badge(lines: Vec<&str>) -> Result<Option<char>, &'static str> {
    if lines.len() != 3 {
        return Err("Invalid number of lines");
    }

    for c in lines[0].chars() {
        if lines[1].contains(c) && lines[2].contains(c) {
            return Ok(Some(c));
        }
    }

    return Ok(None);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = include_str!("../input").lines().collect();
    let point_dict = get_point_dic();
    let mut total_points: i32 = 0;

    for i in 0..lines.len() {
        if i % 3 == 0 || i == 0 {
            // check for out of bounds
            if i + 2 < lines.len() {
                let three_lines = vec![lines[i], lines[i + 1], lines[i + 2]];
                let badge = find_badge(three_lines);
                match badge {
                    Ok(Some(c)) => {
                        let found_points =
                            point_dict.get(&c).ok_or("Invalid letter for points dict")?;
                        total_points += found_points;
                    }
                    Ok(None) => panic!("Error: There was no badge found. Iteration: {}", i),
                    Err(e) => {
                        panic!("Error: There was an issue finding a badge: {}", e);
                    }
                }
            }
        }
    }

    println!("Total points: {}", total_points);
    return Ok(());
}
