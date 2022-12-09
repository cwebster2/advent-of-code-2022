use std::fmt::Display;

use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day7!");
    part_one();
    part_two();
}

#[derive(Debug)]
struct ElfFile<'a> {
    size: u32,
    name: String,
    is_dir: bool,
    children: Vec<&'a mut ElfFile<'a>>,
}

impl Display for ElfFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ElfFile {{ name: {}, size: {}, is_dir: {}, children: {:?} }}", self.name, self.size, self.is_dir, self.children)
    }
}


fn part_one() {
    println!("Part one");

    let mut filesystem = ElfFile{
        size: 0,
        name: "/".to_string(),
        is_dir: true,
        children: Vec::new(),
    };

    let mut current_node = &mut filesystem;

    if let Ok(lines) = read_lines("./inputs/input-day7.txt") {
        for line in lines {
            if let Ok(ip) = line {
                //construct a tree
                println!("{}", ip);
                let mut cmd_tokens = ip.split_whitespace();
                match cmd_tokens.next().unwrap() {
                    "$" => {
                        match cmd_tokens.next().unwrap() {
                            "cd" => {
                                println!("cd");
                                match cmd_tokens.next().unwrap() {
                                    "/" => {
                                        println!("cd /");
                                        current_node = &mut filesystem;
                                    },
                                    &_ => {
                                        let dirname = cmd_tokens.next().unwrap();
                                        for child in current_node.children {
                                            if child.name == dirname {
                                                current_node = &mut child;
                                            }
                                        }
                                    },
                                }
                            },
                            "ls" => println!("ls"),
                            &_ => (),
                            // "ls"
                        }
                    },
                    "dir" => {
                        println!("dir");
                        let dirname = cmd_tokens.next().unwrap();
                        current_node.children.push(ElfFile{
                            size: 0,
                            name: dirname.to_string(),
                            is_dir: true,
                            children: Vec::new(),
                        });
                    },
                    // numbers,
                    c @ &_ => {
                        let size = c.parse::<u32>().unwrap();
                        let name = cmd_tokens.next().unwrap();
                        current_node.children.push(ElfFile{
                            size,
                            name: name.to_string(),
                            is_dir: false,
                            children: Vec::new(),
                        });
                    },
                }
            }
        }
    }
    println!("filesystem: {:#?}", filesystem);
}

fn part_two() {
    println!("Part two");

    if let Ok(lines) = read_lines("./inputs/input-day7.txt") {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
}
