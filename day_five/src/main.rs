use std::cell::RefCell;

#[derive(Debug)]
struct Command(usize, usize, usize);

fn main() {
    let state_file = std::fs::read_to_string("input/state.txt").unwrap();
    let move_file = std::fs::read_to_string("input/test.txt").unwrap();

    let mut stacks: Vec<Vec<char>> = parse_state(&state_file);
    let commands = parse_commands(&move_file);

    // run_commands(&mut stacks, &commands);

    let mut stacks_ref: Vec<RefCell<Vec<char>>> = stacks
        .into_iter()
        .map(|i: Vec<char>| RefCell::new(i))
        .collect();

    run_commands_9001(&mut stacks_ref, &commands);
    println!(
        "{}",
        stacks_ref
            .iter()
            .map(|v| String::from(v.borrow().last().unwrap().to_string()))
            .fold("".to_string(), |p, n| [p, n].join(""))
    );
}

fn run_commands_9001(stacks: &mut Vec<RefCell<Vec<char>>>, commands: &Vec<Command>) {
    for Command(a, b, c) in commands.iter() {
        let mut src = stacks.get(b - 1).unwrap().borrow_mut();
        let (start, end) = (src.len() - a, src.len());
        let mut dest = stacks.get(c - 1).unwrap().borrow_mut();
        for c in src.drain(start..end) {
            dest.push(c);
        }
    }
}
fn run_commands(stacks: &mut Vec<Vec<char>>, commands: &Vec<Command>) {
    for Command(a, b, c) in commands.iter() {
        for _ in 0..*a {
            let val = {
                let src = stacks.get_mut(b - 1).unwrap();
                src.pop().unwrap()
            };
            let dest = stacks.get_mut(c - 1).unwrap();
            dest.push(val);
        }
    }
}

fn compute_size(line: &str) -> usize {
    (line.len() + 1) / 4
}

fn map_index(val: usize) -> usize {
    (val - 1) / 4
}

fn create_vec(input: &String) -> Vec<Vec<char>> {
    let size = compute_size(&input.lines().nth(0).unwrap());
    let mut res: Vec<Vec<char>> = Vec::new();
    for _ in 0..size {
        res.push(Vec::new());
    }
    res
}

fn parse_commands(input: &String) -> Vec<Command> {
    let mut res: Vec<Command> = Vec::new();
    for line in input.lines() {
        let mut nums: Vec<usize> = Vec::new();
        for c in line.split(" ") {
            if c.chars().all(|a| a.is_ascii_digit()) {
                nums.push(c.parse::<usize>().unwrap());
            }
        }
        let (a, b, c) = match &nums[..] {
            [a, b, c] => (a, b, c),
            _ => panic!("Invalid input line, too many numbers"),
        };
        res.push(Command(*a, *b, *c));
    }
    res
}

fn parse_state(input: &String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = create_vec(input);
    for line in input.lines() {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if c.is_ascii_alphabetic() {
                result
                    .get_mut(map_index(i))
                    .unwrap()
                    .push(line.as_bytes()[i] as char);
            }
        }
    }

    result.iter_mut().for_each(|v| v.reverse());
    result
}
