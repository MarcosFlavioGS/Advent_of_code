mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use day1::part1::fd_high_cal_elf;
use day1::part2::fd_top_three;
//-------------------------------
use day2::part1::get_total_score;
use day2::part2::get_true_score;
//-------------------------------
use day3::part1::fd_item_type;
use day3::part2::fd_common_type;
//-------------------------------
use day4::part1::fd_complete_overlap;
use day4::part2::fd_any_overlap;
//-------------------------------
use day5::part1::fd_top_crates;

fn main() {
    println!(
        "This is my repository for solved and solving
        advent of code puzzles."
    );
    println!("Day 1\n*--------------------*");
    println!("Part 1:");
    println!("Answer: {}", fd_high_cal_elf());
    println!("Part 2:");
    println!("Answer: {}", fd_top_three());
    println!("Day 2\n*--------------------*");
    println!("Part 1:");
    println!("Answer: {}", get_total_score());
    println!("Part 2:");
    println!("Answer: {}", get_true_score());
    println!("Day 3\n*--------------------*");
    println!("Part 1:");
    println!("Answer: {}", fd_item_type());
    println!("Part 2:");
    println!("Answer: {}", fd_common_type());
    println!("Day 4\n*--------------------*");
    println!("Part 1:");
    println!("Asnwer: {}", fd_complete_overlap());
    println!("Part 2:");
    println!("Asnwer: {}", fd_any_overlap());
    println!("Day 5\n*--------------------*");
    println!("Part 1:");
    println!("Answer: {:?}", fd_top_crates());
}
