pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|pair_data| {
            let pair_data = pair_data
                .split(",")
                .map(|assigned_sections| {
                    let parsed_vector = assigned_sections
                        .split("-")
                        .map(|section| section.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    parsed_vector[0]..=parsed_vector[1]
                })
                .collect::<Vec<_>>();

            (pair_data[0].clone(), pair_data[1].clone())
        })
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a
                .clone()
                .into_iter()
                .all(|num| range_b.contains(&num));

            let b_contains_a = range_b
                .clone()
                .into_iter()
                .all(|num| range_a.contains(&num));

            a_contains_b || b_contains_a
        })
        .count();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|pair_data| {
            let pair_data = pair_data
                .split(",")
                .map(|assigned_sections| {
                    let parsed_vector = assigned_sections
                        .split("-")
                        .map(|section| section.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    parsed_vector[0]..=parsed_vector[1]
                })
                .collect::<Vec<_>>();

            (pair_data[0].clone(), pair_data[1].clone())
        })
        .filter(|(range_a, range_b)| {
            let a_overlaps_b = range_a
                .clone()
                .into_iter()
                .any(|num| range_b.contains(&num));

            let b_overlaps_a = range_b
                .clone()
                .into_iter()
                .any(|num| range_a.contains(&num));

            a_overlaps_b || b_overlaps_a
        })
        .count();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "4");
    }
}
