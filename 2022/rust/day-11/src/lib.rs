use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Debug)]
enum Operand {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    true_recipient: u64,
    false_recipient: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    fn parse_operand(input: &str) -> IResult<&str, Operand> {
        let (input, value) = alt((
            tag("old").map(|_| Operand::Old),
            complete::u64.map(|v| Operand::Num(v)),
        ))(input)?;
        Ok((input, value))
    }

    let (input, operand_1) = preceded(tag("Operation: new = "), parse_operand)(input)?;
    let (input, operator) = delimited(tag(" "), alt((tag("+"), tag("*"))), tag(" "))(input)?;
    let (input, operand_2) = parse_operand(input)?;

    let operation = match operator {
        "+" => Operation::Add(operand_1, operand_2),
        "*" => Operation::Mul(operand_1, operand_2),
        _ => panic!("unknown operator"),
    };
    Ok((input, operation))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, divisor) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, true_recipient) = delimited(
        multispace1,
        preceded(tag("If true: throw to monkey "), complete::u64),
        multispace1,
    )(input)?;
    let (input, false_recipient) =
        preceded(tag("If false: throw to monkey "), complete::u64)(input)?;

    Ok((
        input,
        Test {
            divisor,
            true_recipient,
            false_recipient,
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _monkey) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, starting_items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = parse_test(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(starting_items),
            operation,
            test,
            touch_count: 0,
        },
    ))
}

impl Monkey {
    /// Inspects the items and returns the current worry level (i.e. next item)
    fn inspect(&mut self) -> u64 {
        self.touch_count += 1;

        let current_item = self.items.pop_front().unwrap();

        let worry_level = match &self.operation {
            Operation::Add(a, b) => {
                let op_1 = match a {
                    Operand::Old => current_item,
                    Operand::Num(num) => *num,
                };

                let op_2 = match b {
                    Operand::Old => current_item,
                    Operand::Num(num) => *num,
                };

                op_1 + op_2
            }
            Operation::Mul(a, b) => {
                let op_1 = match a {
                    Operand::Old => current_item,
                    Operand::Num(num) => *num,
                };

                let op_2 = match b {
                    Operand::Old => current_item,
                    Operand::Num(num) => *num,
                };

                op_1 * op_2
            }
        };

        worry_level / 3
    }

    /// Takes the current worry level and returns the recipient for the current item
    fn test(&self, worry_level: u64) -> u64 {
        if worry_level % self.test.divisor == 0 {
            self.test.true_recipient
        } else {
            self.test.false_recipient
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(multispace1, parse_monkey)(input).unwrap();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let current_monkey = monkeys.get_mut(monkey_index).unwrap();
                let item = current_monkey.inspect();
                let next_monkey_index = current_monkey.test(item);

                monkeys
                    .get_mut(next_monkey_index as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.touch_count);

    monkeys
        .iter()
        .map(|monkey| monkey.touch_count)
        .rev()
        .take(2)
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    42.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "42");
    }
}
