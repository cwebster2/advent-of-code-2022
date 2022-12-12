use std::fmt::{Display, Formatter};

use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day8!");
    part_one();
    part_two();
}

#[derive(Debug, Clone)]
enum Tree {
    Hidden(i32),
    Visible(i32),
    Unknown(i32),
    Uninitizlized
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Hidden(_) => write!(f, " "),
            Tree::Visible(_) => write!(f, "O"),
            Tree::Unknown(c) => write!(f, "{}", c),
            Tree::Uninitizlized => write!(f, " ")
        };
        Ok(())
    }
}

fn print_trees(trees: &Vec<Vec<Tree>>) {
    for row in trees {
        for tree in row {
            print!("{}", tree);
        }
        println!();
    }
}

fn part_one() {
    println!("Part one");

    let mut score = 0;
    let mut xlen = 0;
    let mut ylen = 0;
    //get dimensions
    if let Ok(lines) = read_lines("./inputs/input-day8.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if xlen == 0 {
                    xlen = ip.len();
                }
                ylen += 1;
            }
        }
    }

    let mut trees: Vec<Vec<Tree>> = vec![vec![Tree::Uninitizlized; xlen]; ylen];
    // actually read it now
    let mut ypos = 0;
    let mut xpos = 0;
    if let Ok(lines) = read_lines("./inputs/input-day8.txt") {
        for line in lines {
            if let Ok(ip) = line {
                xpos = 0;
                for c in ip.chars() {
                    trees[ypos][xpos] = Tree::Unknown(c.to_digit(10).unwrap() as i32);
                    xpos += 1;
                }
            }
            ypos += 1;
        }
    }
    print_trees(&trees);
    println!("Score: {}", score);

    // calculate seen along each axis in both pos and neg directions

    let mut threshold: i32 = -1;
    for (y, row) in &mut trees.iter_mut().enumerate() {
        println!("row {}", y);
        threshold = -1;
        for (_, mut tree) in row.iter_mut().enumerate() {
            if let Tree::Unknown(c) = tree {
                if *c > threshold {
                    threshold = *c;
                    println!("New threshold: {}", threshold);
                    tree = &mut Tree::Visible(*c);
                }
            }
        }
    }
    print_trees(&trees);
}

fn part_two() {
    println!("Part two");

    let mut score = 0;
    if let Ok(lines) = read_lines("./inputs/input-day8.txt") {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("Score: {}", score);

}
