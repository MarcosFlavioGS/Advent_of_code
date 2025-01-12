use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fd_item_type() -> u32 {
    let mut input: Vec<(String, String)>;
    let priorities: Vec<u32>;

    input = Vec::new();
    if let Ok(vec) = get_input("inputs/day3.txt") {
        input = vec;
    }
    priorities = get_priorities(input);
    priorities.iter().sum()
}

fn get_input(path: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let file: File;
    let reader: BufReader<File>;
    let mut result: Vec<(String, String)>;

    file = File::open(path)?;
    reader = BufReader::new(file);
    result = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        result.push((
            String::from(&line[..(line.len() / 2)]),
            String::from(&line[(line.len() / 2)..]),
        ));
    }
    Ok(result)
}

fn get_priorities(input: Vec<(String, String)>) -> Vec<u32> {
    let mut result: Vec<u32>;
    result = Vec::new();
    for tup in input {
        let c = get_char(tup.0, tup.1);
        result.push(get_priority(c));
    }
    result
}

fn get_char(a: String, b: String) -> Option<char> {
    a.chars().filter(|&c| b.contains(c)).next()
}

fn get_priority(c: Option<char>) -> u32 {
    let result: u32;
    let char: char = c.unwrap();
    match char.is_uppercase() {
        true => result = char as u32 - 38,
        false => result = char as u32 - 96,
    }
    result
}
