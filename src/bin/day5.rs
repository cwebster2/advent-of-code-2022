use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day5!");
    part_one();
    part_two();
}

fn push_string(stack: &mut Vec<char>, sequence: String) {
    for char in sequence.chars() {
        stack.push(char);
    }
}

fn crates_setup() -> [Vec<char>; 9] {
    let mut stacks: [Vec<char>; 9] = Default::default();

    push_string(&mut stacks[0], "FCPGQR".to_string());
    push_string(&mut stacks[1], "WTCP".to_string());
    push_string(&mut stacks[2], "BHPMC".to_string());
    push_string(&mut stacks[3], "LTQSMPR".to_string());
    push_string(&mut stacks[4], "PHJZVGN".to_string());
    push_string(&mut stacks[5], "DPJ".to_string());
    push_string(&mut stacks[6], "LGPZFJTR".to_string());
    push_string(&mut stacks[7], "NLHCFPTJ".to_string());
    push_string(&mut stacks[8], "GKZQHTCW".to_string());

    stacks
}

fn move_crates(stack: &mut [Vec<char>], count: usize, from: usize, to: usize) {
    for _ in 0..count {
        let c = stack[from-1].pop();
        match c {
            Some(c) => stack[to-1].push(c),
            None => (),
        };
    }
}

fn move_crates_9001(stack: &mut [Vec<char>], count: usize, from: usize, to: usize) {
    let mut crane: Vec<char> = Default::default();
    for _ in 0..count {
        let c = stack[from-1].pop();
        match c {
            Some(c) => crane.push(c),
            None => (),
        };
    }
    while let Some(c) = crane.pop() {
        stack[to-1].push(c);
    }
}

fn print_top_crates(stacks: &[Vec<char>]) {
    for stack in stacks {
        if stack.len() > 0 {
            print!("{}", stack[stack.len() - 1]);
        } else {
            print!(" ");
        }
    }
    println!();
}

fn part_one() {
    println!("Part one");

    let mut stacks: [Vec<char>; 9] = crates_setup();

    if let Ok(lines) = read_lines("./inputs/input-day5.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("move") {
                    let mut parts = ip.split_whitespace();
                    parts.next();
                    let count = parts.next().unwrap().parse::<usize>().unwrap();
                    parts.next();
                    let from = parts.next().unwrap().parse::<usize>().unwrap();
                    parts.next();
                    let to = parts.next().unwrap().parse::<usize>().unwrap();
                    move_crates(&mut stacks, count, from, to);
                }
            }
        }
    }
    print_top_crates(&stacks);
}

fn part_two() {
    println!("Part two");

    let mut stacks: [Vec<char>; 9] = crates_setup();

    if let Ok(lines) = read_lines("./inputs/input-day5.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("move") {
                    let mut parts = ip.split_whitespace();
                    parts.next();
                    let count = parts.next().unwrap().parse::<usize>().unwrap();
                    parts.next();
                    let from = parts.next().unwrap().parse::<usize>().unwrap();
                    parts.next();
                    let to = parts.next().unwrap().parse::<usize>().unwrap();
                    move_crates_9001(&mut stacks, count, from, to);
                }
            }
        }
    }
    print_top_crates(&stacks);

}
