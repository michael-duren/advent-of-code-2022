use day_5::run;
use std::process;

fn main() {
    let lines = include_str!("../input").lines();
    if let Err(e) = run(lines) {
        println!("Error: {:?}", e);
        process::exit(1);
    }
}
