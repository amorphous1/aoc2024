use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> Result<(), std::io::Error> {
    let day01_input = &fs::read_to_string("inputs/day01.input")?;
    println!("{}", day01::part1(day01_input));
    println!("{}", day01::part2(day01_input));

    let day02_input = &fs::read_to_string("inputs/day02.input")?;
    println!("{}", day02::is_safe(day02_input, false));
    println!("{}", day02::is_safe(day02_input, true));

    let day03_input = &fs::read_to_string("inputs/day03.input")?;
    println!("{}", day03::part1(day03_input));
    println!("{}", day03::part2(day03_input));

    let day04_input = &fs::read_to_string("inputs/day04.input")?;
    println!("{}", day04::part1(day04_input));
    println!("{}", day04::part2(day04_input));

    let day05_input = &fs::read_to_string("inputs/day05.input")?;
    println!("{}", day05::day05(day05_input, true));
    println!("{}", day05::day05(day05_input, false));

    Ok(())
}
