use day_5::run;
use std::process;

fn main() {
    let lines = include_str!("../test-input").lines();
    if let Err(e) = run(lines) {
        println!("Error: {:?}", e);
        process::exit(1);
    }
}
