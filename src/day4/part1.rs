use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fd_pairs() -> u32 {
    let mut input: Vec<(String, String)>;

    input = Vec::new();
    if let Ok(result) = get_input("inputs/day4.txt") {
        input = result;
    }
    for tup in input {
        println!("{}, {}", tup.0, tup.1);
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
        let parts: Vec<&str> = line.split(',').collect();
        result.push((parts[0].to_string(), parts[1].to_string()));
    }
    Ok(result)
}
