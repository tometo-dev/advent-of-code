use nom::{
    branch,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Ops {
    AddX(i32),
    Noop,
}

fn parse_addx(input: &str) -> IResult<&str, Ops> {
    let (input, _) = tag("addx ")(input)?;
    let (input, x) = complete::i32(input)?;

    Ok((input, Ops::AddX(x)))
}

fn parse_noop(input: &str) -> IResult<&str, Ops> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Ops::Noop))
}

fn parse_ops(input: &str) -> IResult<&str, Vec<Ops>> {
    let (input, ops) = separated_list1(newline, branch::alt((parse_addx, parse_noop)))(input)?;

    Ok((input, ops))
}

pub fn process_part1(input: &str) -> String {
    let (_, ops) = parse_ops(input).unwrap();

    const MAX_CYCLES: u32 = 230;

    let interested_cycles = vec![20, 60, 100, 140, 180, 220];

    let mut cmd_remaining_cycles = 0;
    let mut current_ops_index: usize = 0;
    let mut to_be_added = 0;

    let mut X = 1;

    let mut signal_strength = 0;

    for i in 1..=MAX_CYCLES {
        if cmd_remaining_cycles == 0 {
            let next_cmd = ops[current_ops_index];
            match next_cmd {
                Ops::Noop => {
                    cmd_remaining_cycles = 0;
                    current_ops_index += 1;
                }
                Ops::AddX(x) => {
                    cmd_remaining_cycles = 1;
                    to_be_added = x;
                }
            }
        } else {
            X += to_be_added;
            to_be_added = 0;
            cmd_remaining_cycles = 0;
            current_ops_index += 1;
        }

        if interested_cycles.iter().any(|&c| c == i) {
            signal_strength += (i as i32) * X;
        }
    }

    signal_strength.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, ops) = parse_ops(input).unwrap();

    const MAX_CYCLES: u32 = 240;

    let mut cmd_remaining_cycles = 0;
    let mut current_ops_index: usize = 0;
    let mut to_be_added = 0;

    let mut X = 1;

    let mut result: String = String::from("");

    for i in 1..=MAX_CYCLES {
        if (X - 1..=X + 1).contains(&((i as i32 - 1) % 40)) {
            result = format!("{}#", result);
        } else {
            result = format!("{}.", result);
        }

        if (i) % 40 == 0 {
            result = format!("{}\n", result);
        }

        if cmd_remaining_cycles == 0 {
            let next_cmd = ops[current_ops_index];
            match next_cmd {
                Ops::Noop => {
                    cmd_remaining_cycles = 0;
                    current_ops_index += 1;
                }
                Ops::AddX(x) => {
                    cmd_remaining_cycles = 1;
                    to_be_added = x;
                }
            }
        } else {
            X += to_be_added;
            to_be_added = 0;
            cmd_remaining_cycles = 0;
            current_ops_index += 1;
        }
    }

    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "13360");
    }
    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
