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

    let mut outcome_score: HashMap<char, usize> = HashMap::new();
    outcome_score.insert('X', 0);
    outcome_score.insert('Y', 3);
    outcome_score.insert('Z', 6);
    let mut beats: HashMap<char, char> = HashMap::new();
    beats.insert('Z', 'X');
    beats.insert('X', 'Y');
    beats.insert('Y', 'Z');

    let mut sum: usize = 0;
    for line in file.lines() {
        if line == "" {
            continue;
        }
        let opp = *move_to_move.get(&line.chars().nth(0).unwrap()).unwrap();
        let outcome = line.chars().nth(2).unwrap();

        sum += outcome_score.get(&outcome).unwrap();

        if outcome == 'Z' {
            sum += move_to_point.get(beats.get(&opp).unwrap()).unwrap();
        } else if outcome == 'Y' {
            sum += move_to_point.get(&opp).unwrap();
        } else {
            sum += move_to_point.get(&beats.get(&beats.get(&opp).unwrap()).unwrap()).unwrap();
        }
    }
    println!("{}", sum);
}
