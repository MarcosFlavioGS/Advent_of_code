mod day1;
mod day2;
mod day3;

use day1::part1::fd_high_cal_elf;
use day1::part2::fd_top_three;
//-------------------------------
use day2::part1::get_total_score;
use day2::part2::get_true_score;
//-------------------------------
use day3::part1::fd_item_type;

fn main() {
    println!(
        "This is my repository for solved and solving\n
        advent of code puzzles."
    );
    println!(
        "Day 1\n*--------------------*"
    );
    println!("Part 1:");
    println!("Answer: {}", fd_high_cal_elf());
    println!("Part 2:");
    println!("Answer: {}", fd_top_three());
    println!(
        "Day 2\n*--------------------*"
    );
    println!("Part 1:");
    println!("Answer: {}", get_total_score());
    println!("Part 2:");
    println!("Answer: {}", get_true_score());
    println!(
        "Day 3\n*--------------------*"
    );
    println!("Part 1:");
    println!("Answer: {}", fd_item_type());
}
