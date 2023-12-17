use std::fmt::Error;

use day_04::part1::process;

fn main() -> Result<(), Error> {
    let file = include_str!("../../input.txt");
    let result = process(file, "Card 100: ".len());
    println!("{}", result);
    Ok(())
}
