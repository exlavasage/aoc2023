use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Part {
    value: u32,
    row: usize,
    col: usize,
}

impl Part {
    fn symbol_adjacent(&self, symbols: &HashSet<(usize, usize)>) -> bool {
        for col in -1isize..((self.value.ilog10() + 2) as isize) {
            for row in -1isize..2 {
                match (
                    self.col.checked_add_signed(col),
                    self.row.checked_add_signed(row),
                ) {
                    (Some(col), Some(row)) => {
                        if symbols.contains(&(col, row)) {
                            return true;
                        }
                    }
                    _ => continue,
                }
            }
        }
        false
    }

    fn is_adjacent(&self, (col, row): (usize, usize)) -> bool {
        let min_col = self.col.checked_add_signed(-1).unwrap_or(0);
        let min_row = self.row.checked_add_signed(-1).unwrap_or(0);
        let max_col = self.col + (self.value.ilog10() as usize) + 1;
        let max_row = self.row + 1;

        min_col <= col && max_col >= col && min_row <= row && max_row >= row
    }
}

fn read() {
    let file = File::open("input/day3.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let mut parts = Vec::new();
    let mut symbols = HashSet::new();
    let mut gears = HashSet::new();

    for (row, line) in reader.lines().filter_map(|line| line.ok()).enumerate() {
        let mut col = 0;
        let line = line.as_bytes();

        while col < line.len() {
            if (line[col] as char).is_digit(10) {
                let mut part = Part { value: 0, row, col };
                while col < line.len() {
                    if let Some(d) = (line[col] as char).to_digit(10) {
                        part.value *= 10;
                        part.value += d;
                    } else {
                        break;
                    }
                    col += 1;
                }
                parts.push(part);
            } else if (line[col] as char) == '.' {
                col += 1
            } else {
                if (line[col] as char) == '*' {
                    gears.insert((col, row));
                }
                symbols.insert((col, row));

                col += 1
            }
        }
    }

    let sum: u32 = parts
        .iter()
        .filter_map(|p| {
            if p.symbol_adjacent(&symbols) {
                Some(p.value)
            } else {
                None
            }
        })
        .sum();

    println!("Sum: {}", sum);

    let sum: u32 = gears
        .iter()
        .filter_map(|(col, row)| {
            let p: Vec<&Part> = parts
                .iter()
                .filter(|p| p.is_adjacent((*col, *row)))
                .collect();
            if p.len() == 2 {
                Some(p[0].value * p[1].value)
            } else {
                None
            }
        })
        .sum();

    println!("Gear Sum: {}", sum);
}

fn main() {
    read();
}
