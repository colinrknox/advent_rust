use std::collections::{HashSet, HashMap};

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


fn get_badge(items: &String) -> usize {
    let mut freq: HashMap<u8, usize> = HashMap::new();
    for item in items.as_bytes() { 
        if let Some(frequency) = freq.get(item) {
            freq.insert(*item, frequency + 1);
        } else {
            freq.insert(*item, 1);
        }
    }

    for (k, v) in freq.iter() {
        if *v == 3 {
            return char_conversion(*k).into();
        }
    }

    return 0;
}


fn char_conversion(val: u8) -> u8 {
    if val < 97 {
        return val - 38u8;
    }
    return val - 96;
}
