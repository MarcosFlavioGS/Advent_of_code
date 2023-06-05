use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_total_score() -> u32 {
    let mut rounds: Vec<Vec<String>>;

    rounds = Vec::new();
    if let Ok(vec) = get_rounds("inputs/day2.txt") {
        rounds = vec;
    }
    println!("{}", rounds[0][0]);
    42
}

fn get_rounds(file_path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let     file: File;
    let     reader: BufReader<File>;
    let mut result: Vec<Vec<String>>;
    let mut line_vector: Vec<String>;

    result = Vec::new();
    line_vector = Vec::new();
    file = File::open(file_path)?;
    reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        match line.is_empty() {
            true => {
                result.push(line_vector);
                line_vector = Vec::new();
            },
            false => {
                let play: String = String::from(line);
                line_vector.push(play);
            }
        }
    }
    if !line_vector.is_empty() {
        result.push(line_vector);
    }
    Ok(result)
}
