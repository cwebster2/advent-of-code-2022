use std::fmt::Display;

use advent_of_code_2022::read_lines;

fn main() {
    println!("Advent of code Day7!");
    part_one();
    part_two();
}

#[derive(Debug)]
struct ElfFile {
    size: u32,
    name: String,
    is_dir: bool,
}

impl ElfFile {
    fn new(size: u32, name: String, is_dir: bool ) -> Self {
        Self {
            size,
            name,
            is_dir
        }
    }
}

impl Display for ElfFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ElfFile {{ name: {}, size: {}, is_dir: {}", self.name, self.size, self.is_dir)
    }
}

#[derive(Debug)]
struct Node<T> {
    data: Box<T>,
    next: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            next: Vec::new(),
            data: Box::new(data),
        }
    }
}

impl Node<ElfFile> {
    fn push(&mut self, node: Node<ElfFile>, path: &Vec<String>)
    {
        // println!("pushing {} to {:?}", node.data.name, path);
        let mut target = self;
        for dir in path {
            // println!("dir: {}", dir);
            let index = target.next.iter().position(|x| x.data.name == *dir);
            match index {
                Some(i) => {
                    // println!("found dir {}", dir);
                    target = &mut target.next[i];
                },
                None => println!("dir {} not found", dir)
            }
        }
        target.next.push(node);
    }
}


fn part_one() {
    println!("Part one");

    let root_node = ElfFile::new(0, "/".to_string(), true);
    let mut filesystem = Node::<ElfFile>::new(root_node);
    let mut current_dir: Vec<String> = Vec::new();

    // construct a tree
    if let Ok(lines) = read_lines("./inputs/input-day7.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut cmd_tokens = ip.split_whitespace();
                match cmd_tokens.next().unwrap() {
                    "$" => {
                        match cmd_tokens.next().unwrap() {
                            "cd" => {
                                match cmd_tokens.next().unwrap() {
                                    "/" => current_dir = Vec::new(),
                                    ".." => {
                                        if current_dir.len() > 0 {
                                            current_dir.pop();
                                        }
                                    },
                                    dirname @ _ => current_dir.push(dirname.to_string()),
                                }
                            },
                            &_ => (),
                        }
                    },
                    "dir" => {
                        let dirname = cmd_tokens.next().unwrap();
                        let newelf = ElfFile::new(0, dirname.to_string(), true,);
                        let filesystem_node: Node<ElfFile> = Node {
                            next: Vec::new(),
                            data: Box::new(newelf),
                        };
                        filesystem.push(filesystem_node, &current_dir);
                    },
                    c @ &_ => {
                        let size = c.parse::<u32>().unwrap();
                        let name = cmd_tokens.next().unwrap();
                        let newelf = ElfFile::new(size, name.to_string(), false,);
                        let filesystem_node: Node<ElfFile> = Node {
                            next: Vec::new(),
                            data: Box::new(newelf),
                        };
                        filesystem.push(filesystem_node, &current_dir);
                    },
                }
            }
        }
    }
    // At this point the filesystem in the data file is constructed.
    println!("filesystem: {:#?}", filesystem);

    // Need to walk the tree and total up the sizes.

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
