use std::{io::BufRead, time::Instant, usize};

type Rucksack = Vec<u8>;

fn input() -> Vec<Rucksack> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect()
}

fn priority(item: u8) -> usize {
    if (b'a'..=b'z').contains(&item) {
        (item - b'a' + 1) as usize
    } else {
        (item - b'A' + 27) as usize
    }
}

fn main() {
    let time = Instant::now();

    let input = input();
    part1(&input);
    part2(&input);

    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &Vec<Rucksack>) {
    let mut input = input.to_vec();

    let result: usize = input
        .iter_mut()
        .filter_map(|rucksack| {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

            compartment1
                .iter()
                .find(|item| compartment2.contains(item))
                .copied()
        })
        .map(priority)
        .sum();

    println!("Part 1 answer: {}", result);
}

fn part2(input: &Vec<Rucksack>) {
    let result: usize = input
        .chunks(3)
        .filter_map(|rucksacks| {
            rucksacks[0]
                .iter()
                .find(|item| rucksacks[1].contains(item) && rucksacks[2].contains(item))
                .copied()
        })
        .map(priority)
        .sum();

    println!("Part 2 answer: {}", result);
}
