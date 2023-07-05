use std::{collections::HashMap, io::BufRead};

fn main() {
    let file = std::fs::read_to_string("input/test.txt").expect("Failed to read input file");

    let mut move_to_move: HashMap<char, char> = HashMap::new();
    move_to_move.insert('A', 'X');
    move_to_move.insert('B', 'Y');
    move_to_move.insert('C', 'Z');

    let mut move_to_point: HashMap<char, usize> = HashMap::new();
    move_to_point.insert('X', 1);
    move_to_point.insert('Y', 2);
    move_to_point.insert('Z', 3);

    let mut beats: HashMap<char, char> = HashMap::new();
    beats.insert('X', 'Z');
    beats.insert('Y', 'X');
    beats.insert('Z', 'Y');

    let mut sum: usize = 0;
    for line in file.lines() {
        if line == "" {
            continue;
        }
        let opp = *move_to_move.get(&line.chars().nth(0).unwrap()).unwrap();
        let me = line.chars().nth(2).unwrap();

        if opp == me {
            sum += 3;
        } else if *beats.get(&opp).unwrap() != me {
            sum += 6;
        }
        sum += move_to_point.get(&me).unwrap();
    }
    println!("{}", sum);
}
