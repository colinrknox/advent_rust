use std::collections::{HashSet, HashMap};

fn main() {
    // let file_contents = std::fs::read_to_string("input/test.txt").unwrap();
    let file_contents = std::fs::read_to_string("input/small_test.txt").unwrap();
    let mut sum: usize = 0;
    // for line in file_contents.lines() {
    //     sum += get_priority(&line.to_string());
    // }
    let mut group: Vec<String> = Vec::new();
    for line in file_contents.lines() {
        group.push(line.to_string());
        if group.len() == 3 {
            println!("sum: {}", sum);
            sum += get_badge(group.join("")) as usize;
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


fn get_badge(items: Vec<String>) -> u8 {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for elf in items {
        for item in elf.as_bytes() { 
            let item_convert = *item as char;
            let f = if let Some(frequency) = freq.get(&item_convert) {
                frequency + 1
            } else {
                1 
            };
            freq.insert(item_convert, f);
        }
    }

    for (k, v) in freq.iter() {
        println!("{} showed up 3 times!", k);
        if *v == 3 {
            return char_conversion(*k as u8);
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
