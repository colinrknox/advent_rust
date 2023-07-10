use std::collections::{HashSet, HashMap};

fn main() {
    // let file_contents = std::fs::read_to_string("input/test.txt").unwrap();
    let file_contents = std::fs::read_to_string("input/test.txt").unwrap();
    let mut sum: usize = 0;
    // for line in file_contents.lines() {
    //     sum += get_priority(&line.to_string());
    // }
    let mut group: Vec<String> = Vec::new();
    for line in file_contents.lines() {
        group.push(line.to_string());
        if group.len() == 3 {
            sum += get_badge(&group) as usize;
            group.clear();
        }
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
    for j in half..len {
        let c = line.as_bytes()[j] as char;
        if chars.contains(&c) {
            let base = if c.is_ascii_uppercase() {
                38
            } else {
                96
            };
            chars.remove(&c);
            result += c as usize - base;
        }
    }
    result
}


fn get_badge(items: &Vec<String>) -> u8 {
    let mut chars = build_set(&items[0]);
    for elf in &items[1..] {
        let intersect = build_set(&elf);
        chars.retain(|x| intersect.contains(x));
    }
    for v in chars {
        return char_conversion(v as u8);
    }
    return 0;
}

fn build_set(items: &String) -> HashSet<char> {
    let mut res: HashSet<char> = HashSet::new();
    for item in items.as_bytes() {
        res.insert(*item as char);
    }
    res
}


fn char_conversion(val: u8) -> u8 {
    if val < 97 {
        return val - 38u8;
    }
    return val - 96;
}
