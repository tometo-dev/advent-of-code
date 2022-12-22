use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::DiGraphMap};

pub fn process_part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // find the starting position's index
    let start_index = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == 'S' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    // find the ending position's index
    let end_index = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == 'E' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    // replace 'S' and 'E' with their respective elevation

    let grid = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    other => *other,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let edges = (0..(grid[0].len() as i32))
        .cartesian_product(0..(grid.len() as i32))
        .flat_map(|(x, y)| {
            let neighbours = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

            // check the traversal criteria for each of the neighbouring cells
            neighbours
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.1 as usize).and_then(|row| {
                        row.get(cell.0 as usize).and_then(|&valid_cell| {
                            let current_cell = grid[y as usize][x as usize];
                            if current_cell as u8 + 1 >= valid_cell as u8 {
                                Some(((x, y, current_cell), (cell.0, cell.1, valid_cell)))
                            } else {
                                None
                            }
                        })
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(edges);

    let result = dijkstra(
        &graph,
        (start_index.0, start_index.1, 'a'),
        Some((end_index.0, end_index.1, 'z')),
        |_| 1,
    );

    result[&(end_index.0, end_index.1, 'z')].to_string()
}

pub fn process_part2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // find the ending position's index
    let end_index = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == 'E' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    // replace 'S' and 'E' with their respective elevation

    let grid = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    other => *other,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let edges = (0..(grid[0].len() as i32))
        .cartesian_product(0..(grid.len() as i32))
        .flat_map(|(x, y)| {
            let neighbours = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

            // check the traversal criteria for each of the neighbouring cells
            neighbours
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.1 as usize).and_then(|row| {
                        row.get(cell.0 as usize).and_then(|&valid_cell| {
                            let current_cell = grid[y as usize][x as usize];
                            if current_cell as u8 + 1 >= valid_cell as u8 {
                                Some(((x, y, current_cell), (cell.0, cell.1, valid_cell)))
                            } else {
                                None
                            }
                        })
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // reverse the edges and traverse from the ending position
    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(edges.iter().map(|&(a, b)| (b, a)));

    let result = dijkstra(&graph, (end_index.0, end_index.1, 'z'), None, |_| 1);

    // find all the valid 'a' positions
    let result = result
        .iter()
        .filter_map(
            |(node, &cost)| {
                if node.2 == 'a' {
                    Some(cost)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    // return the one with the lowest cost
    result.iter().sorted().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "31");
    }

    #[test]

    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "29");
    }
}
