use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_true_score() -> u32 {
    let mut rounds: Vec<Vec<String>>;
    let mut scores: Vec<u32>;
    let mut result: u32;

    rounds = Vec::new();
    if let Ok(vec) = get_rounds("inputs/day2.txt") {
        rounds = vec;
    }
    for round in &mut rounds {
        update_round(round);
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

fn update_round(round: &mut Vec<String>) {
    for i in 0..round.len() {
        let mut chars = round[i].chars().filter(|&c| c != ' ');
        let     a = chars.next().unwrap();
        let     b = chars.next().unwrap();
        round[i] = round[i].replace(b, make_decision(a, b));
    }
}

fn make_decision(a: char, b: char) -> &'static str {
    "X"
}

fn round_score(round: Vec<String>) -> u32 {
    let mut score: u32 = 0;

    for line in round {
        let mut chars = line.chars().filter(|&c| c != ' ');
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let play = (first, second);
        score += analyse(play);
    }
    score
}

fn analyse(play: (char, char)) -> u32 {
    let mut result = check_victory(play.0, play.1);
    match play.1 {
        'X' => result += 1,
        'Y' => result += 2,
        'Z' => result += 3,
        _ => println!("Do Nothing"),
    }
    result
}

fn check_victory(a: char, b: char) -> u32 {
    let mut result: u32 = 0;
    match a {
       'A' => {
           match b {
               'X' => result = 3,
               'Y' => result = 6,
               'Z' => result = 0,
               _ => println!("Nothing special"),
           }
        },
        'B' => {
            match b {
                'X' => result = 0,
                'Y' => result = 3,
                'Z' => result = 6,
                _ => println!("Nothing special"),
            }
        },
        'C' => {
            match b {
                'X' => result = 6,
                'Y' => result = 0,
                'Z' => result = 3,
                _ => println!("Nothing special"),
            }
        },
        _ => println!("Nothing special"),
    }
    result
}
