use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::{char, digit1, multispace0, multispace1};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, terminated};
use nom::{bytes::complete::tag, sequence::tuple};
use nom::{IResult, Parser};
use std::{io::BufRead, time::Instant};

type Stack = Vec<char>;

#[derive(Debug, PartialEq, Eq)]
struct Command {
    count: usize,
    source: usize,
    target: usize,
}

#[derive(Debug)]
struct Input {
    stacks: Vec<Stack>,
    commands: Vec<Command>,
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (i, c) = delimited(char('['), take(1usize), char(']'))(input)?;
    Ok((i, c.chars().nth(0)))
}

fn parse_crate_option(input: &str) -> IResult<&str, Option<char>> {
    let parse_none = tag("   ").map(|_| None);
    alt((parse_crate, parse_none))(input)
}

fn parse_crate_lines(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    many0(terminated(
        separated_list0(char(' '), parse_crate_option),
        char('\n'),
    ))(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Stack>> {
    let stack_number_line = delimited(
        multispace0,
        separated_list0(multispace1, digit1),
        multispace0,
    );

    let (i, lines) = terminated(parse_crate_lines, stack_number_line)(input)?;

    let count = match lines.get(0) {
        Some(v) => v.len(),
        None => return Ok((i, vec![])),
    };

    let mut stack = lines
        .into_iter()
        .fold(vec![vec![]; count], |mut stacks, line| {
            for (stack, item) in stacks.iter_mut().zip(line) {
                if let Some(c) = item {
                    stack.push(c);
                }
            }

            stacks
        });

    for s in stack.iter_mut() {
        s.reverse();
    }

    Ok((i, stack))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (i, (_, count, _, source, _, target)) = tuple((
        tag("move "),
        digit1,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ))(input)?;

    let count: usize = count.parse().unwrap();
    let source: usize = source.parse().unwrap();
    let target: usize = target.parse().unwrap();

    Ok((
        i,
        Command {
            count,
            source: source - 1,
            target: target - 1,
        },
    ))
}

fn parse_command_list(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list0(char('\n'), parse_command)(input)
}

fn input() -> Result<Input> {
    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let input = input.join("\n");

    let (_, (stacks, commands)) =
        tuple((parse_stacks, parse_command_list))(&input).map_err(|err| err.to_owned())?;

    Ok(Input { stacks, commands })
}

fn part1(input: &Input) {
    let mut stacks = input.stacks.clone();

    for command in &input.commands {
        let index = stacks[command.source].len() - command.count;
        let mut moved = stacks[command.source].split_off(index);
        moved.reverse();
        stacks[command.target].append(&mut moved);
    }

    let result: String = stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect();
    println!("Part 1 answer: {}", result);
}

fn part2(input: &Input) {
    let mut stacks = input.stacks.clone();

    for command in &input.commands {
        let index = stacks[command.source].len() - command.count;
        let mut moved = stacks[command.source].split_off(index);
        stacks[command.target].append(&mut moved);
    }

    let result: String = stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect();
    println!("Part 2 answer: {}", result);
}

fn main() -> Result<()> {
    let time = Instant::now();

    let input = input()?;
    part1(&input);
    part2(&input);

    println!("Time elapsed is {:?}", time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_parsing() {
        let input = "[N]";

        let (i, c) = parse_crate(input).unwrap();

        assert_eq!("", i);
        assert_eq!('N', c.unwrap());
    }

    #[test]
    fn crate_lines_parsing() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
";

        let (i, lines) = parse_crate_lines(input).unwrap();

        assert_eq!("", i);
        assert_eq!(
            vec![
                vec![None, Some('D'), None],
                vec![Some('N'), Some('C'), None],
                vec![Some('Z'), Some('M'), Some('P')]
            ],
            lines
        );
    }

    #[test]
    fn stack_parsing() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

";

        let (i, stacks) = parse_stacks(input).unwrap();

        assert_eq!("", i);
        assert_eq!(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']], stacks);
    }

    #[test]
    fn command_parsing() {
        let input = r"move 3 from 1 to 3";

        let (i, command) = parse_command(input).unwrap();

        assert_eq!("", i);
        assert_eq!(
            Command {
                count: 3,
                source: 1 - 1,
                target: 3 - 1
            },
            command
        );
    }

    #[test]
    fn command_list_parsing() {
        let input = r"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let (i, command_list) = parse_command_list(input).unwrap();

        assert_eq!("", i);
        assert_eq!(
            vec![
                Command {
                    count: 1,
                    source: 2 - 1,
                    target: 1 - 1
                },
                Command {
                    count: 3,
                    source: 1 - 1,
                    target: 3 - 1
                },
                Command {
                    count: 2,
                    source: 2 - 1,
                    target: 1 - 1
                },
                Command {
                    count: 1,
                    source: 1 - 1,
                    target: 2 - 1
                }
            ],
            command_list
        );
    }
}
