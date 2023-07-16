struct Command(usize, usize, usize);

fn main() {
    let state_file = std::fs::read_to_string("input/state.txt").unwrap();
    let move_file = std::fs::read_to_string("input/test.txt").unwrap();

    let mut stacks: Vec<Vec<char>> = parse_state(&state_file);

    println!("{:?}", stacks);
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
