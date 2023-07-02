
fn main() {
    let file_contents: String = std::fs::read_to_string(std::path::Path::new("input/elves.txt")).expect("should have been able to read the input file");

    print!("{}", file_contents);

    let parsed_contents: Vec<Vec<&str>> = parse_input(&file_contents); 
}

fn parse_input(input: &String) -> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = vec!();
    let mut set: Vec<&str> = vec![];
    for line in input.lines() {
        print!("{}", line);
        if line == "" {
            print!("line is equal to newline");
            result.push(set.clone());
            set = vec!();
        }
        set.push(line);
    }
    return result
}
