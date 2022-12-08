use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day1!");

    let mut calories = 0;
    let mut calorie_counts = Vec::new();

    if let Ok(lines) = read_lines("./inputs/input-day1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let calorie = ip.parse::<i32>();
                match calorie {
                    Ok(calorie) => {
                        calories += calorie;
                    }
                    Err(_) => {
                        calorie_counts.push(calories);
                        calories = 0;
                    }
                }
            }
        }
    }
    calorie_counts.sort_by(|a, b| b.cmp(a));
    println!("{}", calorie_counts[0] + calorie_counts[1] + calorie_counts[2]);
}
