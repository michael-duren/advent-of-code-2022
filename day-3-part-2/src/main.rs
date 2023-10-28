use std::collections::HashMap;

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
fn find_badge(lines: Vec<&str>) -> Option<char> {
    for c in lines.get(0).unwrap().chars() {
        if lines.get(1).unwrap().contains(c) && lines.get(2).unwrap().contains(c) {
            return Some(c);
        }
    }

    return None;
}

fn main() {
    let lines: Vec<&str> = include_str!("../input").lines().collect();
    let point_dict = get_point_dic();
    let mut total_points: i32 = 0;

    for i in 0..lines.len() {
        if i % 3 == 0 || i == 0 {
            if i + 2 < lines.len() {
                let three_lines = vec![lines[i], lines[i + 1], lines[i + 2]];
                let badge = find_badge(three_lines);
                println!("Badge: {:?}", badge);
                total_points += point_dict.get(&badge.unwrap()).unwrap();
            }
        }
    }

    println!("Total points: {}", total_points);
}
