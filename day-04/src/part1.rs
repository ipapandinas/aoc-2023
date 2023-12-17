pub fn process(input: &str, offset: usize) -> i32 {
    input
        .lines()
        .filter_map(|x| {
            if let Some(pos) = x.find(" | ") {
                let (reference_str, game_str) = x.split_at(pos);
                let reference = reference_str[offset..].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                let game = game_str[" | ".len()..].split_whitespace().map(|x| x.parse::<i32>().unwrap());

                let mut count = 0;
                for num in game {
                    let occurences = reference.iter().filter(|&&x| x == num).count();
                    match occurences {
                        0 => (),
                        _ => {
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    return Some(2_i32.pow(count - 1));
                }

                return Some(0)
            }
            None
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use super::*;

    #[test]
    fn test_process() -> Result<(), Error> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input, "Card 1: ".len()));
        Ok(())
    }
}
