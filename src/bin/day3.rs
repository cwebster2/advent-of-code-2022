use std::collections::HashSet;

use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day2!");

    let mut score = 0;
    if let Ok(lines) = read_lines("./inputs/input-day3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let len = ip.len();
                let parts = ip.split_at(len/2);
                let p1: HashSet<_> = parts.0.chars().collect();
                let p2: HashSet<_> = parts.1.chars().collect();
                let mut i = p1.intersection(&p2);
                score += match i.next() {
                    Some(c @ 'a'..='z') => { *c as u8 - b'a' + 1 },
                    Some(c @ 'A'..='Z') => { *c as u8 - b'A' + 27 },
                    _ => { 0 },
                } as i32;
            }
        }
    }
    println!("Total score: {}", score);

    let mut score = 0;
    if let Ok(mut lines) = read_lines("./inputs/input-day3.txt") {
        while let [ Some(Ok(a)), Some(Ok(b)), Some(Ok(c)) ] = [ lines.next(), lines.next(), lines.next() ] {
            let p1: HashSet<_> = a.chars().collect();
            let p2: HashSet<_> = b.chars().collect();
            let p3: HashSet<_> = c.chars().collect();
            let i = p1.intersection(&p2).cloned().collect();
            let mut b = p3.intersection(&i);
            score += match b.next() {
                Some(c @ 'a'..='z') => { *c as u8 - b'a' + 1 },
                Some(c @ 'A'..='Z') => { *c as u8 - b'A' + 27 },
                _ => { 0 },
            } as i32;
        }
    }
    println!("Total score: {}", score);
}
