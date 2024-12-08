use std::fs;
use std::time::SystemTime;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() -> Result<(), std::io::Error> {
    let day01_input = &fs::read_to_string("inputs/day01.txt")?;
    println!("{}", day01::part1(day01_input));
    println!("{}", day01::part2(day01_input));

    let day02_input = &fs::read_to_string("inputs/day02.txt")?;
    println!("{}", day02::is_safe(day02_input, false));
    println!("{}", day02::is_safe(day02_input, true));

    let day03_input = &fs::read_to_string("inputs/day03.txt")?;
    println!("{}", day03::part1(day03_input));
    println!("{}", day03::part2(day03_input));

    let day04_input = &fs::read_to_string("inputs/day04.txt")?;
    println!("{}", day04::part1(day04_input));
    println!("{}", day04::part2(day04_input));

    let day05_input = &fs::read_to_string("inputs/day05.txt")?;
    println!("{}", day05::day05(day05_input, true));
    println!("{}", day05::day05(day05_input, false));

    let day06_input = &fs::read_to_string("inputs/day06.txt")?;
    print_and_time(|| day06::visited_positions(day06_input).len());
    print_and_time(|| day06::loop_obstructions(day06_input));

    let day07_input = &fs::read_to_string("inputs/day07.txt")?;
    print_and_time(|| day07::sum_solvable_equations(day07_input, false));
    print_and_time(|| day07::sum_solvable_equations(day07_input, true));

    let day08_input = &fs::read_to_string("inputs/day08.txt")?;
    print_and_time(|| day08::unique_antinodes(day08_input));
    Ok(())
}

fn print_and_time<F: Fn() -> T, T: std::fmt::Display>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    println!("{} (took {:?})", result, start.elapsed().unwrap());
    result
}
