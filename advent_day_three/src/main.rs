use std::collections::HashSet;

fn main() {
    let file_contents = std::fs::read_to_string("input/test.txt").unwrap();
    
    let mut sum: usize = 0;
    for line in file_contents.lines() {
        sum += get_priority(&line.to_string());
    }
    println!("{}", sum);
}

fn get_priority(line: &String) -> usize {

    let mut result: usize = 0;
    let mut chars: HashSet<char> = HashSet::new();
    let half = line.len() / 2;
    let len = line.len();
    for i in 0..half {
        chars.insert(line.as_bytes()[i] as char);
    }
    dbg!(&chars);
    for j in half..len {
        let c = line.as_bytes()[j] as char;
        if chars.contains(&c) {
            let base = if c.is_ascii_uppercase() {
                38
            } else {
                96
            };
            chars.remove(&c);
            println!("dupe: {}", c);
            println!("c: {}, base: {}", c as usize, base);
            result += c as usize - base;
        }
    }
    result
}
