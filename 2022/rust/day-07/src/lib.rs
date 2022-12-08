use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Cd {
    Root,
    Up,
    Into(String),
}

#[derive(Debug)]
enum Operation {
    Cd(Cd),
    Ls(Vec<Files>),
}

#[derive(Debug)]
enum Files {
    File { size: u32 },
    Dir(String),
}

fn parse_cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;

    let operation = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        other => Operation::Cd(Cd::Into(other.to_string())),
    };

    Ok((input, operation))
}

fn parse_file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;

    Ok((input, Files::File { size }))
}

fn parse_dir(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, Files::Dir(name.to_string())))
}

fn parse_ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((parse_file, parse_dir)))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, operations) = separated_list1(newline, alt((parse_ls, parse_cd)))(input)?;

    Ok((input, operations))
}

fn calculate_sizes(
    (mut context, mut sizes): (Vec<String>, BTreeMap<Vec<String>, u32>),
    operation: &Operation,
) -> (Vec<String>, BTreeMap<Vec<String>, u32>) {
    match operation {
        Operation::Cd(Cd::Root) => {
            context.push(String::from(""));
        }
        Operation::Cd(Cd::Up) => {
            context.pop();
        }
        Operation::Cd(Cd::Into(file)) => {
            context.push(file.clone());
        }
        Operation::Ls(files) => {
            let sum = files
                .iter()
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();

            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|size| *size += sum)
                    .or_insert(sum);
            }
        }
    }

    (context, sizes)
}

pub fn process_part1(input: &str) -> String {
    let (_input, operations) = parse_operations(input).unwrap();

    let (_, sizes) = operations
        .iter()
        .fold((vec![], BTreeMap::new()), calculate_sizes);

    sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, operations) = parse_operations(input).unwrap();

    let (_, sizes) = operations
        .iter()
        .fold((vec![], BTreeMap::new()), calculate_sizes);

    let total_space = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get(&vec![String::from("")]).unwrap();

    let free_space = total_space - used_space;

    let need_to_be_freed_space = needed_space - free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_be_freed_space)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    valid_dirs.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "24933642");
    }
}
