pub fn process_part1(input: &str) -> String {
    let input_vec = input.chars().collect::<Vec<char>>();
    let mut index = 0;

    let window = 4;

    for idx in 0..input_vec.len() {
        if idx < window {
            continue;
        }

        let sequence = &input_vec.as_slice()[idx - window..idx];
        if !(1..window).any(|i| sequence[i..].contains(&sequence[i - 1])) {
            index = idx;
            break;
        }
    }
    index.to_string()
}

pub fn process_part2(input: &str) -> String {
    let input_vec = input.chars().collect::<Vec<char>>();
    let mut index = 0;

    let window = 14;

    for idx in 0..input_vec.len() {
        if idx < window {
            continue;
        }

        let sequence = &input_vec.as_slice()[idx - window..idx];
        if !(1..window).any(|i| sequence[i..].contains(&sequence[i - 1])) {
            index = idx;
            break;
        }
    }
    index.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "19");
    }
}
