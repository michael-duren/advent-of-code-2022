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
    let file = std::fs::read_to_string("puzzle_input").unwrap(); // read file, returning Ok value
    let mut all_carries: Vec<Vec<u64>> = vec![]; // create a vector of vectors to store each elves carry
    let mut current_carry: Vec<u64> = vec![]; // current elf carry before a new line occurs

    for line in file.lines() {
        match line.is_empty() {
            true => {
                all_carries.push(current_carry); // push current carry to all carries
                current_carry = vec![]; // reset current carry, all_carries has ownership of previous value
            }
            false => {
                let parsed_num: u64 = line.parse().unwrap(); // parse line to u64
                current_carry.push(parsed_num); // push parsed number to current carry
            }
        }
    }

    if !current_carry.is_empty() {
        all_carries.push(current_carry);
    } // push last carry to all carries

    let mut summed_calories: Vec<u64> = all_carries.iter().map(|c| c.iter().sum()).collect();
    summed_calories.sort_by(|a, b| b.cmp(a));
    let top_three_summed: u64 = summed_calories.iter().take(3).sum();

    return top_three_summed;
}
