use std::{fs, path::Path};

pub fn read_input(input: &str) -> String {
    let base_path = Path::new("inputs");
    fs::read_to_string(base_path.join(input)).expect("Failed to read file")
}

pub fn pretty_print_matrix(v: &Vec<Vec<&str>>) {
    for row in v {
        for c in row {
            print!("{} ", c);
        }
        println!("");
    }
}
