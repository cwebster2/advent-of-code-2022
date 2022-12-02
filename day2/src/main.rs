use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Add;

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

impl Add<Shape> for Shape {
    type Output = i32;
    fn add(self, other: Shape) -> i32 {
        self as i32 + other as i32
    }
}
impl Add<Outcome> for Outcome {
    type Output = i32;
    fn add(self, other: Outcome) -> i32 {
        self as i32 + other as i32
    }
}
impl Add<Shape> for Outcome {
    type Output = i32;
    fn add(self, other: Shape) -> i32 {
        self as i32 + other as i32
    }
}
impl Add<Outcome> for Shape {
    type Output = i32;
    fn add(self, other: Outcome) -> i32 {
        self as i32 + other as i32
    }
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
                        score += Shape::Scissors + Outcome::Lose;
                    },
                    ("A", "Y") => {
                        // rock DRAW
                        score += Shape::Rock + Outcome::Draw;
                    },
                    ("A", "Z") => {
                        // rock WIN
                        score += Shape::Paper + Outcome::Win;
                    },
                    ("B", "X") => {
                        // paper LOSE
                        score += Shape::Rock + Outcome::Lose;
                    },
                    ("B", "Y") => {
                        // paper DRAW
                        score += Shape::Paper + Outcome::Draw;
                    },
                    ("B", "Z") => {
                        // paper WIN
                        score += Shape::Scissors + Outcome::Win;
                    },
                    ("C", "X") => {
                        // scissor LOSE
                        score += Shape::Paper + Outcome::Lose;
                    },
                    ("C", "Y") => {
                        // scissor DRAW
                        score += Shape::Scissors + Outcome::Draw;
                    },
                    ("C", "Z") => {
                        // scissor WIN
                        score += Shape::Rock + Outcome::Win;
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
