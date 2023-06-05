mod day1;
mod day2;

use day1::part1::fd_high_cal_elf;
use day1::part2::fd_top_three;
use day2::part1::get_total_score;

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
}
