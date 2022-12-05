use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{self, alpha1, digit1, multispace1, newline},
        streaming::space1,
    },
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, parsed_value) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match parsed_value {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    // construct the vertical crate stacks
    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];

    for _ in 0..=crates_horizontal.len() {
        crates_vertical.push(vec![]);
    }

    for vec in crates_horizontal.iter().rev() {
        for (i, crate_) in vec.iter().enumerate() {
            crates_vertical[i].push(*crate_);
        }
    }

    // filter out the None values
    let final_crates = crates_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|c| *c).collect())
        .collect::<Vec<Vec<&str>>>();

    Ok((input, (final_crates, moves)))
}

struct Move {
    number: u32,
    from: u32,
    to: u32,
}

fn move_crate(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            number,
            from: from - 1,
            to: to - 1,
        },
    ))
}

pub fn process_part1(input: &str) -> String {
    let (_, (mut crates, moves)) = crates(input).unwrap();

    for Move { number, from, to } in moves.iter() {
        let len = crates[*from as usize].len();
        for crate_ in crates[*from as usize]
            .drain((len - (*number as usize))..)
            .rev()
            .collect::<Vec<&str>>()
            .iter()
        {
            crates[*to as usize].push(crate_);
        }
    }

    let top_crates: String = crates
        .iter()
        .map(|c| match c.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    top_crates
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "CMZ");
    }
}
