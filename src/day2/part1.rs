use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_total_score() -> u32 {
    let mut rounds: Vec<Vec<String>>;
    let mut scores: Vec<u32>;
    let mut result: u32;

    rounds = Vec::new();
    if let Ok(vec) = get_rounds("inputs/day2.txt") {
        rounds = vec;
    }
    scores = Vec::new();
    for round in rounds {
        scores.push(round_score(round));
    }
    result = 0;
    for score in scores {
        result += score;
    }
    result
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

fn round_score(round: Vec<String>) -> u32 {
    let mut score: u32;

    score = 0;
    for line in round {
        let mut chars = line.chars().filter(|&c| c != ' ');
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let play = (first, second);
        score = analyse(play);
    }
    score
}

fn analyse(play: (char, char)) -> u32 {
    42
}
