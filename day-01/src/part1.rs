pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let nums = x
                .chars()
                .filter(|x| x.is_digit(10))
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            match nums.as_slice() {
                [x, .., y] => x * 10 + y,
                [x] => x * 10 + x,
                _ => 0,
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn test_process() -> Result<(), Error> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, process(input));
        Ok(())
    }
}
