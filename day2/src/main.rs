use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    println!("Advent of code Day2!");


    let mut score = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split_whitespace();
                match (split.next().unwrap(), split.next().unwrap()) {
                    ("A", "X") => {
                        // rock LOSE
                        score += Shape::Scissors as i32 + Outcome::Lose as i32;
                    },
                    ("A", "Y") => {
                        // rock DRAW
                        score += Shape::Rock as i32 + Outcome::Draw as i32;
                    },
                    ("A", "Z") => {
                        // rock WIN
                        score += Shape::Paper as i32 + Outcome::Win as i32;
                    },
                    ("B", "X") => {
                        // paper LOSE
                        score += Shape::Rock as i32 + Outcome::Lose as i32;
                    },
                    ("B", "Y") => {
                        // paper DRAW
                        score += Shape::Paper as i32 + Outcome::Draw as i32;
                    },
                    ("B", "Z") => {
                        // paper WIN
                        score += Shape::Scissors as i32 + Outcome::Win as i32;
                    },
                    ("C", "X") => {
                        // scissor LOSE
                        score += Shape::Paper as i32 + Outcome::Lose as i32;
                    },
                    ("C", "Y") => {
                        // scissor DRAW
                        score += Shape::Scissors as i32 + Outcome::Draw as i32;
                    },
                    ("C", "Z") => {
                        // scissor WIN
                        score += Shape::Rock as i32 + Outcome::Win as i32;
                    }
                    (&_, &_) => {
                        println!("Invalid input");
                    }
                }
            }
        }
    }
    println!("Score: {}", score);
}
