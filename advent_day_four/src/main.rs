enum Elf {
    Assignment(usize, usize),
}

struct Pair(Elf, Elf);

fn main() {
    let file_contents = std::fs::read_to_string("input/test.txt").unwrap();
    let example = "2-4,6-8";
    let elves = parse_input(file_contents);
    let mut total = 0;
    for elf in elves {
        if is_contained(elf) {
            total += 1;
        }
    }
    println!("{}", total);
}

fn parse_input(input: String) -> Vec<Pair> {
    let mut res = Vec::new();
    for line in input.lines() {
        let mut vals = Vec::new();
        for elf in line.split(",") {
            let mut nums: Vec<usize> = elf.split("-").map(|x| x.parse::<usize>().unwrap()).collect();
            vals.append(&mut nums);
        }
        res.push(Pair(Elf::Assignment(vals[0], vals[1]), Elf::Assignment(vals[2], vals[3])));
    }
    res
}

fn is_contained(pair: Pair) -> bool {
    false
}
