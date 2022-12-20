use std::collections::BTreeSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn parse_move(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, dir) = alt((
        separated_pair(complete::char('U'), tag(" "), complete::i32)
            .map(|(_, steps)| vec![Move::Up; steps as usize]),
        separated_pair(complete::char('D'), tag(" "), complete::i32)
            .map(|(_, steps)| vec![Move::Down; steps as usize]),
        separated_pair(complete::char('L'), tag(" "), complete::i32)
            .map(|(_, steps)| vec![Move::Left; steps as usize]),
        separated_pair(complete::char('R'), tag(" "), complete::i32)
            .map(|(_, steps)| vec![Move::Right; steps as usize]),
    ))(input)?;
    Ok((input, dir))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (_, moves) = separated_list1(newline, parse_move)(input)?;
    let moves = moves.into_iter().flatten().collect();
    Ok((input, moves))
}

fn move_head(current_pos: &mut (i32, i32), next_move: Move) {
    match next_move {
        Move::Up => current_pos.1 += 1,
        Move::Down => current_pos.1 -= 1,
        Move::Left => current_pos.0 -= 1,
        Move::Right => current_pos.0 += 1,
    };
}

fn move_tail(head: (i32, i32), tail: &mut (i32, i32)) -> bool {
    // cacluate chebyshev's distance between the two points
    let delta_x = head.0 - tail.0;
    let delta_y = head.1 - tail.1;

    let distance = std::cmp::max(delta_x.abs(), delta_y.abs());

    let mut has_changed = false;

    if distance > 1 {
        let delta_x_signum = delta_x.signum();
        let delta_y_signum = delta_y.signum();
        if delta_x_signum != 0 {
            tail.0 += delta_x_signum;
            has_changed = true;
        }
        if delta_y_signum != 0 {
            tail.1 += delta_y_signum;
            has_changed = true;
        }
    }

    has_changed
}

pub fn process_part1(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();

    let mut tail_moves: BTreeSet<(i32, i32)> = BTreeSet::new();
    tail_moves.insert((0, 0));

    let mut head = (0, 0);
    let mut tail = (0, 0);

    for current_move in moves {
        move_head(&mut head, current_move);
        if move_tail(head, &mut tail) {
            tail_moves.insert(tail);
        }
    }

    tail_moves.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();

    let mut tail_moves: BTreeSet<(i32, i32)> = BTreeSet::new();

    const NUMBER_0F_KNOTS: usize = 10;

    let mut rope_knots = vec![(0, 0); NUMBER_0F_KNOTS];
    tail_moves.insert(*rope_knots.last().unwrap());

    for current_move in moves {
        move_head(&mut rope_knots[0], current_move);
        for i in 1..NUMBER_0F_KNOTS - 1 {
            move_tail(rope_knots[i - 1], &mut rope_knots[i]);
        }
        if move_tail(
            rope_knots[NUMBER_0F_KNOTS - 2],
            &mut rope_knots[NUMBER_0F_KNOTS - 1],
        ) {
            tail_moves.insert(rope_knots[NUMBER_0F_KNOTS - 1]);
        }
    }

    tail_moves.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "1");
    }
}
