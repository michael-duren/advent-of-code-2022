fn main() {
    determine_calories();
    let top_3 = determine_top_three_elves();
    println!("{top_3}");
}

fn determine_calories() -> u64 {
    let file = std::fs::read_to_string("puzzle_input").unwrap();
    let mut most_calories: u64 = 0;
    let mut current_carry: u64 = 0; // current elf determination before a new line occurs

    for line in file.lines() {
        if line.is_empty() {
            if current_carry > most_calories {
                most_calories = current_carry;
            } // compare current carry with most calories
            current_carry = 0;
        } else {
            let parsed_num: u64 = line.parse().unwrap();
            current_carry += parsed_num;
        }
    }

    return most_calories;
}

fn determine_top_three_elves() -> u64 {
    let file = std::fs::read_to_string("puzzle_input").unwrap();
    let mut all_carries: Vec<Vec<u64>> = vec![];
    let mut current_carry: Vec<u64> = vec![]; // current elf determination before a new line occurs

    for line in file.lines() {
        match line.is_empty() {
            true => {
                all_carries.push(current_carry);
                current_carry = vec![];
            }
            false => {
                let parsed_num: u64 = line.parse().unwrap();
                current_carry.push(parsed_num);
            }
        }
    }

    // println!(":?", all_carries);

    let mut summed_calories: Vec<u64> = all_carries.iter().map(|c| c.iter().sum()).collect();
    summed_calories.sort_by(|a, b| b.cmp(a));
    let top_3_summed: u64 = summed_calories[(summed_calories.len() - 4)..summed_calories.len()]
        .iter()
        .sum();

    return top_3_summed;
}
