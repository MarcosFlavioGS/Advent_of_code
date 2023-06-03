mod day1;
use day1::part1::fd_high_cal_elf;
use day1::part2::fd_top_three;

fn main() {
    println!(
        "This is my repository for solved and solving\n
        advent of code puzzles."
    );
    println!(
        "Day 1\n*--------------------*"
    );
    println!("Part 1:");
    fd_high_cal_elf();
    println!("Part 2:");
    fd_top_three();
}
