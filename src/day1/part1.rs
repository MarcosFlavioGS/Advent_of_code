use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day1::Elf;

pub fn fd_high_cal_elf() -> u32 {
    let mut elfs: Vec<Elf>;
    let     calorie_vec: Vec<Vec<u32>>;

    elfs = Vec::new();
    if let Ok(vec) = calories_vectorize("inputs/day1.txt") {
        calorie_vec = vec;
    } else {
        panic!("Failed to read from file.");
    }
    for elf in calorie_vec {
        elfs.push(Elf::Calories(elf));
    }
    //println!("{}", elfs[0].sum()); // Exemple of how to get a sum of an elf's calories
    elfs.iter().map(|x| x.sum()).max().unwrap()
}

fn calories_vectorize(file_path: &str) -> Result<Vec<Vec<u32>>, std::io::Error> {
    let     file = File::open(file_path)?;
    let     reader = BufReader::new(file);
    let mut result: Vec<Vec<u32>>;
    let mut line_vector: Vec<u32>;

    result = Vec::new();
    line_vector = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        match line.is_empty() {
            true => {
                result.push(line_vector);
                line_vector = Vec::new();
            },
            false => {
                let number:u32 = line
                    .trim()
                    .parse()
                    .expect("failed to parse number");
                line_vector.push(number);
            }
        }
    }

    if !line_vector.is_empty() {
        result.push(line_vector);
    }
    Ok(result)
}
