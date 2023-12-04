use std::fmt::Error;

use day_01::part1::process;

fn main() -> Result<(), Error> {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{}", result);
    Ok(())
}
