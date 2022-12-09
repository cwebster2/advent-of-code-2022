use std::collections::HashSet;

use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day6!");
    part_one();
    part_two();
}
fn part_one() {
    println!("Part one");

    let mut score = 0;
    if let Ok(lines) = read_lines("./inputs/input-day6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut window: Vec::<char> = Default::default();
                for c in ip.chars() {
                    window.push(c);
                    score += 1;
                    if window.len() < 4 {
                        continue;
                    }
                    if window.len() > 4 {
                        window.remove(0);
                    }
                    let mut uniq = HashSet::new();
                    let success = window.clone().into_iter().all(move |x| uniq.insert(x));
                    if success {
                        break;
                    }
                }
            }
        }
    }
    println!("Score: {}", score);

}

fn part_two() {
    println!("Part two");

    let mut score = 0;
    if let Ok(lines) = read_lines("./inputs/input-day6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut window: Vec::<char> = Default::default();
                for c in ip.chars() {
                    window.push(c);
                    score += 1;
                    if window.len() < 14 {
                        continue;
                    }
                    if window.len() > 14 {
                        window.remove(0);
                    }
                    let mut uniq = HashSet::new();
                    let success = window.clone().into_iter().all(move |x| uniq.insert(x));
                    if success {
                        break;
                    }
                }
            }
        }
    }
    println!("Score: {}", score);

}
