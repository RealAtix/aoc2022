use std::{io::BufRead, ops::RangeInclusive, time::Instant};

type Assignment = RangeInclusive<usize>;

fn parse_range(range: &str) -> RangeInclusive<usize> {
    let mut parts = range.split('-');
    let start: usize = parts.next().unwrap().parse().unwrap();
    let end: usize = parts.next().unwrap().parse().unwrap();

    RangeInclusive::new(start, end)
}

fn input() -> Vec<[Assignment; 2]> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| -> [RangeInclusive<usize>; 2] {
            let mut parts = line.split(',');

            let first = parts.next().unwrap();
            let first = parse_range(first);

            let second = parts.next().unwrap();
            let second = parse_range(second);
            [first, second]
        })
        .collect()
}

fn main() {
    let time = Instant::now();

    let input = input();
    part1(&input);
    part2(&input);

    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &Vec<[Assignment; 2]>) {
    let result = input
        .iter()
        .filter(|[first, second]| {
            let c0 = first.contains(second.start()) && first.contains(second.end());
            let c1 = second.contains(first.start()) && second.contains(first.end());

            c0 || c1
        })
        .count();

    println!("Part 1 answer: {}", result);
}

fn part2(input: &Vec<[Assignment; 2]>) {
    let result = input
        .iter()
        .filter(|[first, second]| {
            let c0 = first.contains(second.start()) || first.contains(second.end());
            let c1 = second.contains(first.start()) || second.contains(first.end());

            c0 || c1
        })
        .count();

    println!("Part 1 answer: {}", result);
}
