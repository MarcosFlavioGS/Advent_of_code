pub mod part1;
pub mod part2;

pub use part1::fd_high_cal_elf;
pub use part2::fd_top_three;

pub enum Elf {
    Calories(Vec<u32>),
}

impl Elf {
    fn sum(&self) -> u32 {
        match self {
            Elf::Calories(calories) => {
                let mut sum: u32;

                sum = 0;
                for cal in calories {
                    sum += cal;
                }
                sum
            }
        }
    }
}
