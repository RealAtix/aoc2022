use std::{io::BufRead, time::Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn new(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Illegal character"),
        }
    }

    fn score(&self) -> usize {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum GameRound {
    OpponentWins { opponent: Shape, player: Shape },
    Draw { opponent: Shape, player: Shape },
    PlayerWins { opponent: Shape, player: Shape },
}

impl GameRound {
    fn play(opponent: Shape, player: Shape) -> Self {
        use Shape::*;
        if opponent == player {
            Self::Draw { opponent, player }
        } else {
            match (&opponent, &player) {
                (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => {
                    Self::OpponentWins { opponent, player }
                }
                (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => {
                    Self::PlayerWins { opponent, player }
                }
                _ => unreachable!(),
            }
        }
    }

    fn score(&self) -> usize {
        match &self {
            Self::OpponentWins {
                opponent: _,
                player,
            } => 0 + player.score(),
            Self::Draw {
                opponent: _,
                player,
            } => 3 + player.score(),
            Self::PlayerWins {
                opponent: _,
                player,
            } => 6 + player.score(),
        }
    }
}

fn input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect()
}

fn main() {
    let time = Instant::now();

    let input = input();
    part1(&input);
    part2(&input);

    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &Vec<Vec<char>>) {
    let result: usize = input
        .iter()
        .map(|hands| {
            let opponent = Shape::new(*hands.first().unwrap());
            let player = Shape::new(*hands.last().unwrap());
            GameRound::play(opponent, player).score()
        })
        .sum();
    println!("Part 1 answer: {}", result);
}

fn part2(input: &Vec<Vec<char>>) {
    let result: usize = input
        .iter()
        .map(|hands| {
            use Shape::*;
            let opponent = Shape::new(*hands.first().unwrap());
            let cheating_player = match *hands.last().unwrap() {
                'X' => {
                    // Lose
                    match opponent {
                        Rock => Scissors,
                        Paper => Rock,
                        Scissors => Paper,
                    }
                }
                'Y' => {
                    // Draw
                    opponent
                }
                'Z' => {
                    // Win
                    match opponent {
                        Rock => Paper,
                        Paper => Scissors,
                        Scissors => Rock,
                    }
                }
                _ => unreachable!(),
            };
            GameRound::play(opponent, cheating_player).score()
        })
        .sum();
    println!("Part 1 answer: {}", result);
}
