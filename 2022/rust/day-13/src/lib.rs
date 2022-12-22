use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Num(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(left), Self::List(right)) => left == right,
            (Self::Num(left), Self::Num(right)) => left == right,
            (Self::List(left), Self::Num(right)) => left == &vec![Packet::Num(*right)],
            (Self::Num(left), Self::List(right)) => &vec![Packet::Num(*left)] == right,
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(left), Packet::Num(right)) => left.cmp(right),
            (Packet::Num(left), Packet::List(right)) => vec![Packet::Num(*left)].cmp(right),
            (Packet::List(left), Packet::Num(right)) => left.cmp(&vec![Packet::Num(*right)]),
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = alt((
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u32.map(|num| Packet::Num(num)),
    ))(input)?;

    Ok((input, packet))
}

fn parse_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(parse_packet, newline, parse_packet)(input)
}

fn parse_signal(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(tag("\n\n"), parse_pair)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, signal) = parse_signal(input).unwrap();

    let result = signal
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| match a.cmp(b) {
            std::cmp::Ordering::Less => Some(i + 1),
            std::cmp::Ordering::Greater => None,
            std::cmp::Ordering::Equal => panic!("Elements are equal??"),
        })
        .collect::<Vec<_>>();

    result.iter().sum::<usize>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, signal) = parse_signal(input).unwrap();

    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);

    // dbg!(&signal);

    let mut result = signal
        .iter()
        .flat_map(|(a, b)| [a, b])
        .chain([&packet_2, &packet_6])
        .collect::<Vec<_>>();

    result.sort();

    let index_2 = result
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&packet_2)
        .unwrap();

    let index_6 = result
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&packet_6)
        .unwrap();

    // dbg!(result);

    ((index_2.0 + 1) * (index_6.0 + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "140");
    }
}
