pub mod part1;
pub mod part2;

pub use part1::fd_item_type;
pub use part2::fd_common_type;

pub enum Group {
    Rucksack(Vec<String>),
}

impl Group {
    fn priority(&self) -> u32 {
        match self {
            Group::Rucksack(rucksack) => {
                42
            }
        }
    }
}
