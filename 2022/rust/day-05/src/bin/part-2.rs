use std::fs;

use day_05::process_part2;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&file));
}
