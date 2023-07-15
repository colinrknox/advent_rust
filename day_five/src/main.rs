fn main() {
    let state_file = std::fs::read_to_string("input/test.txt").unwrap();
    let move_file = std::fs::read_to_string("input/test.txt").unwrap();

    let mut stacks: Vec<Vec<char>> = parse_state(&state_file);

    println!("small change");
}

fn compute_size(line: &String) -> usize {
    (line.len() + 1) / 4
}

fn map_index(val: usize) -> usize {
    (val - 1) / 4
}

fn create_vec(input: &String) -> Vec<Vec<char>> {
    let size = compute_size(&input.lines().nth(0).unwrap().to_string());
    let mut res: Vec<Vec<char>> = Vec::new();
    for _ in 0..size {
        res.push(Vec::new());
    }
    res
}

fn parse_state(input: &String) -> Vec<Vec<char>> {
    let result = create_vec(input);
    for line in input.lines() {
        for index in line.match_indices("[A-Za-z]") {
            result
                .get(map_index(index.0))
                .unwrap()
                .push(line.as_bytes()[index.0] as char);
        }
    }

    result
}
