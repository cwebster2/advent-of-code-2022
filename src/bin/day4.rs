use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day4!");
    part_one();
    part_two();
}
fn part_one() {
    println!("Part one");

    let mut score = 0;
    if let Ok(lines) = read_lines("./inputs/input-day4.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // convert "A-B,C-D" to [[A, B], [C, D]]
                let assignments = ip.split(",").map(|ranges| {
                    ranges.split("-").map(|n| i32::from_str_radix(n, 10).unwrap_or(0)).collect::<Vec<i32>>()
                }).collect::<Vec<Vec<i32>>>();
                // is one completely contained within the other?
                if (assignments[0][0] <= assignments[1][0] && assignments[0][1] >= assignments[1][1]) ||
                (assignments[1][0] <= assignments[0][0] && assignments[1][1] >= assignments[0][1]) {
                    score += 1;
                }
            }
        }
    }
    println!("Score: {}", score);

}

fn part_two() {
    println!("Part two");

    let mut score = 0;
    if let Ok(lines) = read_lines("./inputs/input-day4.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // convert "A-B,C-D" to [[A, B], [C, D]]
                let assignments = ip.split(",").map(|ranges| {
                    ranges.split("-").map(|n| i32::from_str_radix(n, 10).unwrap_or(0)).collect::<Vec<i32>>()
                }).collect::<Vec<Vec<i32>>>();
                // is there any overlap at all?
                if assignments[0][1] >= assignments[1][0] && assignments[0][0] <= assignments[1][1] {
                    score += 1;
                }
            }
        }
    }
    println!("Score: {}", score);

}
