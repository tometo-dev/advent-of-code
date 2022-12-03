#![feature(iter_array_chunks)]

use std::collections::HashMap;

pub fn process_part1(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .map(|rucksack| {
            let len = rucksack.len() / 2;
            let compartment_1 = &rucksack[..len];
            let compartment_2 = &rucksack[len..];

            let common_item = compartment_1
                .chars()
                .find(|c| compartment_2.contains(*c))
                .unwrap();
            letter_scores.get(&common_item).unwrap()
        })
        .sum::<usize>();
    result
}

pub fn process_part2(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_item = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();

            letter_scores.get(&common_item).unwrap()
        })
        .sum::<usize>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = process_part1(&INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_works() {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = process_part2(&INPUT);
        assert_eq!(result, 70);
    }
}
