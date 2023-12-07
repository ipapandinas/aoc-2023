pub fn process(input: &str, raw_rgb_cubes: (i32, i32, i32)) -> i32 {
    input
        .lines()
        .filter_map(|x| {
            if let Some(pos) = x.find(": ") {
                let (game_str, sets) = x.split_at(pos);
                if let Ok(game) = game_str["Game ".len()..].parse::<i32>() {
                    let mut valid = true;

                    for set in sets[2..].split(";") {
                        let mut rgb_cubes = (0, 0, 0);

                        for item in set.split(", ") {
                            let mut parts = item.split_whitespace();
                            if let (Some(value_str), Some(color)) = (parts.next(), parts.next()) {
                                if let Ok(value) = value_str.parse::<i32>() {
                                    match color {
                                        "blue" => rgb_cubes.2 += value,
                                        "red" => rgb_cubes.0 += value,
                                        "green" => rgb_cubes.1 += value,
                                        _ => return None,
                                    }
                                } else {
                                    return None;
                                }
                            }
                        }

                        if rgb_cubes.0 > raw_rgb_cubes.0
                            || rgb_cubes.1 > raw_rgb_cubes.1
                            || rgb_cubes.2 > raw_rgb_cubes.2
                        {
                            valid = false;
                            break;
                        };
                    }

                    if valid {
                        return Some(game);
                    }
                }
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
        let rgb_cubes = (12, 13, 14);
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input, rgb_cubes));
        Ok(())
    }
}
