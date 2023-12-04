use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Default, Clone, Debug)]
struct Collection {
    red: u32,
    green: u32,
    blue: u32,
}

impl Collection {
    fn possible_pull(&self, other: &Collection) -> bool {
        if other.green > self.green {
            false
        } else if other.red > self.red {
            false
        } else if other.blue > self.blue {
            false
        } else {
            true
        }
    }

    fn power(&self) -> u32 {
        self.green * self.red * self.blue
    }
}

#[derive(Default, Debug)]
struct Game {
    id: u32,
    pulls: Vec<Collection>,
}

impl Game {
    fn possible(&self, setup: &Collection) -> bool {
        self.pulls.iter().all(|pull| setup.possible_pull(pull))
    }

    fn min_collection(&self) -> Collection {
        let mut collection = self.pulls[0].clone();

        for pull in &self.pulls[1..] {
            collection.green = std::cmp::max(collection.green, pull.green);
            collection.blue = std::cmp::max(collection.blue, pull.blue);
            collection.red = std::cmp::max(collection.red, pull.red);
        }

        collection
    }
}

fn parse_pull(input: &str) -> Collection {
    let mut collection = Collection::default();
    for count in input.trim().split(",") {
        let count = count.trim();
        let (number, color) = count.split_once(" ").unwrap();
        let number = number.parse::<u32>().unwrap();
        match color {
            "blue" => collection.blue = number,
            "red" => collection.red = number,
            "green" => collection.green = number,
            _ => panic!(),
        }
    }

    collection
}

fn parse_line(input: &str) -> Game {
    let (game, pulls) = input.split_once(": ").unwrap();
    let id = game
        .split(" ")
        .filter_map(|w| w.parse::<u32>().ok())
        .collect::<Vec<u32>>()[0];

    let pulls = pulls.split(";").map(|pull| parse_pull(pull)).collect();

    Game { id, pulls }
}

fn possible(setup: Collection) -> u32 {
    let file = File::open("input/day2.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let sum: u32 = reader
        .lines()
        .filter_map(|p| p.ok())
        .map(|line| {
            let game = parse_line(&line);
            let possible = game.possible(&setup);
            println!("line: {} possible: {}", line, possible);
            if possible {
                game.id
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {}", sum);
    sum
}

fn power() -> u32 {
    let file = File::open("input/day2.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let sum: u32 = reader
        .lines()
        .filter_map(|p| p.ok())
        .map(|line| {
            let game = parse_line(&line);
            game.min_collection().power()
        })
        .sum();
    println!("Sum: {}", sum);
    sum
}

fn main() {
    let setup = Collection {
        red: 12,
        green: 13,
        blue: 14,
    };

    possible(setup);
    power();
}
