use std::fmt::Error;

use day_02::part1::process;

fn main() -> Result<(), Error> {
    let file = include_str!("../../input1.txt");
    let rgb_cubes = (12, 13, 14);
    let result = process(file, rgb_cubes);
    println!("{}", result);
    Ok(())
}
