use day_4_part_2::{FromStr, IdRange, Overlaps};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let pairs = include_str!("../input").lines().collect::<Vec<_>>();
    let mut total: u32 = 0;

    for pair in pairs {
        let ranges = pair.split(",").collect::<Vec<&str>>();

        let a = IdRange::from_str(ranges[0])?;
        let b = IdRange::from_str(ranges[1])?;

        let result = a.overlaps_with_other(&b);

        if result {
            total += 1
        };
    }

    println!("total is {}", total);

    return Ok(());
}
