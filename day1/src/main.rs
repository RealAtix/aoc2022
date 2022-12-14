use itertools::Itertools;
use std::{io::BufRead, time::Instant};

// Another way
fn input_old() -> Vec<usize> {
    std::io::stdin()
        .lock()
        .lines()
        .batching(|it| {
            Some(
                it.map_while(|line| line.unwrap().parse::<usize>().ok())
                    .sum::<usize>(),
            )
            .filter(|sum| sum > &0)
        })
        .collect()
}

fn input() -> Vec<usize> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|v| v.unwrap().parse::<usize>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
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

fn part1(input: &Vec<usize>) {
    let result = input.iter().max().unwrap_or(&0);
    println!("Part 1 answer: {:?}", result);
}

fn part2(input: &Vec<usize>) {
    let result: usize = input.iter().sorted().rev().take(3).sum();
    println!("Part 2 answer: {:?}", result);
}
