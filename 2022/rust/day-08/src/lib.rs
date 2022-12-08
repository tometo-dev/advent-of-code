fn check_visibility(i: &usize, j: &usize, grid: &Vec<Vec<u32>>) -> bool {
    // visibility in order top, right, bottom, left
    let mut visible = (true, true, true, true);

    // check on right
    for k in j + 1..grid[*i].len() {
        if grid[*i][*j] <= grid[*i][k] {
            visible.1 = false;
            break;
        }
    }

    // check on left
    for k in (0..*j).rev() {
        if grid[*i][*j] <= grid[*i][k] {
            visible.3 = false;
            break;
        }
    }

    // check on top
    for k in (0..*i).rev() {
        if grid[*i][*j] <= grid[k][*j] {
            visible.0 = false;
            break;
        }
    }

    // check on bottom
    for k in i + 1..grid.len() {
        if grid[*i][*j] <= grid[k][*j] {
            visible.2 = false;
            break;
        }
    }

    visible.0 || visible.1 || visible.2 || visible.3
}

pub fn process_part1(input: &str) -> String {
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut count = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if check_visibility(&i, &j, &grid) {
                count += 1;
            }
        }
    }

    (grid.len() * 2 + grid[0].len() * 2 - 4 + count).to_string()
}

fn calculate_scenic_score(i: &usize, j: &usize, grid: &Vec<Vec<u32>>) -> usize {
    // visibility in order top, right, bottom, left
    let mut visibility_distance = (1, 1, 1, 1);

    // check on right
    for k in j + 1..grid[*i].len() {
        visibility_distance.1 = grid[*i].len() - j - 1;
        if grid[*i][*j] <= grid[*i][k] {
            visibility_distance.1 = k - j;
            break;
        }
    }

    // check on left
    for k in (0..*j).rev() {
        visibility_distance.3 = *j;
        if grid[*i][*j] <= grid[*i][k] {
            visibility_distance.3 = j - k;
            break;
        }
    }

    // check on top
    for k in (0..*i).rev() {
        visibility_distance.0 = *i;
        if grid[*i][*j] <= grid[k][*j] {
            visibility_distance.0 = i - k;
            break;
        }
    }

    // check on bottom
    for k in i + 1..grid.len() {
        visibility_distance.2 = grid.len() - i - 1;
        if grid[*i][*j] <= grid[k][*j] {
            visibility_distance.2 = k - i;
            break;
        }
    }

    let result = visibility_distance.0
        * visibility_distance.1
        * visibility_distance.2
        * visibility_distance.3;

    result
}

pub fn process_part2(input: &str) -> String {
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut max_scenic_score = 1;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            let score = calculate_scenic_score(&i, &j, &grid);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    max_scenic_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "8");
    }
}
