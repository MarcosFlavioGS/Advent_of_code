use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fd_complete_overlap() -> u32 {
    let mut input: Vec<(String, String)> = Vec::new();

    if let Ok(result) = get_input("inputs/day4.txt") {
        input = result;
    }
    input.iter().filter(|x| check_contain(x)).count() as u32
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

fn check_contain(tup: &(String, String)) -> bool {
    let asignment1: Vec<&str>;
    let asignment2: Vec<&str>;
    let range1: (u32, u32);
    let range2: (u32, u32);

    asignment1 = tup.0.split('-').collect();
    asignment2 = tup.1.split('-').collect();
    range1 = (asignment1[0].parse().expect("Fail"), asignment1[1].parse().expect("Fail"));
    range2 = (asignment2[0].parse().expect("Fail"), asignment2[1].parse().expect("Fail"));
    if range1.0 <= range2.0 && range1.1 >= range2.1 {
        return true;
    } else if range2.0 <= range1.0 && range2.1 >= range1.1 {
        return true;
    }
    false
}
