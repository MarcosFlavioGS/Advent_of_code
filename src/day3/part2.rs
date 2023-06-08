use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day3::Group;

pub fn fd_common_type() -> u32 {
    let mut input: Vec<String>;
    let mut groups: Vec<Group>;
    let mut priorities: Vec<u32>;

    input = Vec::new();
    if let Ok(vec) = get_input("inputs/day3.txt") {
        input = vec;
    }
    groups = Vec::new();
    init_groups(&mut groups, input);
    priorities = Vec::new();
    for group in groups {
        priorities.push(group.priority());
    }
    priorities.iter().sum()
}

fn get_input(path: &str) -> Result<Vec<String>, std::io::Error> {
    let     file: File;
    let     reader: BufReader<File>;
    let mut result: Vec<String>;

    file = File::open(path)?;
    reader = BufReader::new(file);
    result = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        result.push(line);
    }
    Ok(result)
}

fn init_groups(groups: &mut Vec<Group>, input: Vec<String>) {
    let mut vec: Vec<String>;

    vec = Vec::new();
    for line in input {
        if vec.len() == 3 {
            groups.push(Group::Rucksack(vec));
            vec = Vec::new();
            vec.push(line);
        } else {
            vec.push(line);
        }
    }
    groups.push(Group::Rucksack(vec));
}
