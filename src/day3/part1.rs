use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fd_item_type() -> u32 {
    let mut input: Vec<(String, String)>;

    input = Vec::new();
    if let Ok(vec) = get_input("inputs/day3.txt") {
        input = vec;
    }
    42
}

fn get_input(path: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let     file: File;
    let     reader: BufReader<File>;
    let mut result: Vec<(String, String)>;

    file = File::open(path)?;
    reader = BufReader::new(file);
    result = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let len = line.len();
        result.push((String::from(&line[..(len / 2)]), String::from(&line[(len / 2)..])));
    }
    Ok(result)
}
