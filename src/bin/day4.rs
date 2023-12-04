use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Default, Debug)]
struct Game {
    id: u32,
    winners: HashSet<u32>,
    choices: HashSet<u32>,
}

impl Game {
    fn value(&self) -> u32 {
        2u32.pow(self.winners()) / 2
    }

    fn winners(&self) -> u32 {
        self.winners.intersection(&self.choices).count() as u32
    }
}

fn parse_line(input: &str) -> Game {
    let (game, numbers) = input.split_once(": ").unwrap();
    let id = game
        .split(" ")
        .filter_map(|w| w.parse::<u32>().ok())
        .collect::<Vec<u32>>()[0];

    let (winners, choices) = numbers.split_once("|").unwrap();

    Game {
        id,
        winners: winners
            .trim()
            .split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect(),
        choices: choices
            .trim()
            .split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect(),
    }
}

fn add(map: &mut HashMap<u32, u32>, k: &u32, v: u32) -> u32 {
    let c = *map.get(k).unwrap_or(&0);
    map.insert(*k, v + c);

    v + c
}

fn winner() -> u32 {
    let file = File::open("input/day4.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let mut cards = HashMap::new();

    let sum: u32 = reader
        .lines()
        .filter_map(|p| p.ok())
        .map(|line| {
            let game = parse_line(&line);

            let count = add(&mut cards, &game.id, 1);

            for i in 0..game.winners() {
                add(&mut cards, &(game.id + i + 1), count);
            }

            game.value()
        })
        .sum();
    println!("Sum: {}", sum);

    let sum: u32 = cards.iter().map(|(_k, v)| v).sum();
    println!("Card Sum: {}", sum);
    sum
}

fn main() {
    winner();
}
