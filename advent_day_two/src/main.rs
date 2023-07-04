use std::{collections::HashMap, io::BufRead};

fn main() {
    println!("Hello, world!");
    let file = std::fs::read_to_string("input/small_test.txt").expect("Failed to read input file");

    let mut move_to_point: HashMap<char, usize> = HashMap::new();
    move_to_point.insert('X', 1);
    move_to_point.insert('Y', 2);
    move_to_point.insert('Z', 3);

    let mut beats: HashMap<char, char> = HashMap::new();
    beats.insert('A', 'Z');
    beats.insert('B', 'X');
    beats.insert('C', 'Y');

    let mut sum: usize = 0;
    for line in file.lines() {
        let opp = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        println!("opp: {}, me: {}", opp, me);
        if opp == me {
            sum += 3;
        } else if *beats.get(&opp).unwrap() != me {
            sum += 6;
        }
        sum += move_to_point.get(&me).unwrap();
    }
    println!("{}", sum);
}
