use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let file = std::fs::read_to_string("input/test.txt");

    let mut move_to_point: HashMap<&str, usize> = HashMap::new();
    move_to_point.insert("X", 1);
    move_to_point.insert("Y", 2);
    move_to_point.insert("Z", 3);

    let mut beats: HashMap<&str, &str> = HashMap::new();
    beats.insert("A", "Z");
    beats.insert("B", "X");
    beats.insert("C", "Y");
}
