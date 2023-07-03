use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Eq, PartialEq)]
struct Elf {
    pub contents: Vec<usize>,
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            contents: vec![],
        }
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.contents.iter().sum::<usize>().cmp(&other.contents.iter().sum()).then_with(|| self.contents.len().cmp(&other.contents.len()))
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file_contents: String = std::fs::read_to_string(std::path::Path::new("input/elves.txt")).expect("should have been able to read the input file");

    let parsed_contents: Vec<Vec<&str>> = parse_input(&file_contents); 

    let elves: Vec<Elf> = convert_parsed(&parsed_contents);

    let mut heap: BinaryHeap<Elf> = BinaryHeap::from_iter(elves.into_iter());

    let mut sum = 0;
    let mut i = 0;
    while let Some(elf) = heap.pop() {
        if i > 2 {
            break;
        }
        sum += elf.contents.iter().sum::<usize>();
        i += 1;
    }
    println!("{}", sum);

    if let Some(elf) = heap.peek() {
        println!("{}", elf.contents.iter().sum::<usize>());
    }
}

fn parse_input(input: &String) -> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = vec!();
    let mut set: Vec<&str> = vec![];
    for line in input.lines() {
        if line == "" {
            result.push(set.clone());
            set = vec!();
        }
        set.push(line);
    }
    if !set.is_empty() {
        result.push(set.clone());
    }
    result
}

fn convert_parsed(parsed: &Vec<Vec<&str>>) -> Vec<Elf> {
    let mut result: Vec<Elf> = vec![];
    for ele in parsed.iter() {
        let mut elf = Elf::new();
        for amount in ele.iter() {
            match amount.parse::<usize>() {
                Ok(val) => elf.contents.push(val),
                Err(_e) => {},
            }
        }
        result.push(elf);
    }
    result
}
