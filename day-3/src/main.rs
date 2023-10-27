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

fn main() {
    let lines = include_str!("../input").lines();
    let point_dict = get_point_dic();
    let mut total_points: i32 = 0;

    for line in lines {
        // TODO: Split the lines in half
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];

        for char in first_half.chars() {
            if second_half.contains(char) {
                let point = point_dict.get(&char);
                total_points += point.unwrap();
                break;
            }
        }
    }

    println!("Total Points: {}", total_points);
}
