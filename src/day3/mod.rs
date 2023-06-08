pub mod part1;
pub mod part2;

pub use part1::fd_item_type;
pub use part2::fd_common_type;

#[derive(Debug)]
pub enum Group {
    Rucksack(Vec<String>),
}

impl Group {
    fn priority(&self) -> u32 {
        let result: u32;
        match self {
            Group::Rucksack(rucksack) => {
                let mut c: char = 'A';
                let     alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
                for char in alphabet.chars() {
                    let is_present0 = rucksack[0].contains(char);
                    let is_present1 = rucksack[1].contains(char);
                    let is_present2 = rucksack[2].contains(char);
                    if is_present0 && is_present1 && is_present2 {
                        c = char;
                    }
                }
                match c.is_uppercase() {
                    true => result = c as u32 - 38,
                    false => result = c as u32 - 96,
                }
            }
        }
        result
    }
}
