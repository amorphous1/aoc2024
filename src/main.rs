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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() -> Result<(), std::io::Error> {
    let day01_input = &fs::read_to_string("inputs/day01.txt")?;
    print_and_time("day 1 part 1", || day01::part1(day01_input));
    print_and_time("day 1 part 2", || day01::part2(day01_input));

    let day02_input = &fs::read_to_string("inputs/day02.txt")?;
    print_and_time("day 2 part 1", || day02::is_safe(day02_input, false));
    print_and_time("day 2 part 2", || day02::is_safe(day02_input, true));

    let day03_input = &fs::read_to_string("inputs/day03.txt")?;
    print_and_time("day 3 part 1", || day03::part1(day03_input));
    print_and_time("day 3 part 2", || day03::part2(day03_input));

    let day04_input = &fs::read_to_string("inputs/day04.txt")?;
    print_and_time("day 4 part 1", || day04::part1(day04_input));
    print_and_time("day 4 part 2", || day04::part2(day04_input));

    let day05_input = &fs::read_to_string("inputs/day05.txt")?;
    print_and_time("day 5 part 1", || day05::day05(day05_input, true));
    print_and_time("day 5 part 2", || day05::day05(day05_input, false));

    let day06_input = &fs::read_to_string("inputs/day06.txt")?;
    print_and_time("day 6 part 1",|| day06::visited_positions(day06_input).len());
    print_and_time("day 6 part 2",|| day06::loop_obstructions(day06_input));

    let day07_input = &fs::read_to_string("inputs/day07.txt")?;
    print_and_time("day 7 part 1", || day07::sum_solvable_equations(day07_input, false));
    print_and_time("day 7 part 2", || day07::sum_solvable_equations(day07_input, true));

    let day08_input = &fs::read_to_string("inputs/day08.txt")?;
    print_and_time("day 8 part 1", || day08::unique_antinodes(day08_input));
    print_and_time("day 8 part 2", || day08::unique_antinodes2(day08_input));

    let day09_input = &fs::read_to_string("inputs/day09.txt")?;
    print_and_time("day 9 part 1", || day09::checksum(day09_input));
    print_and_time("day 9 part 2", || day09::checksum2(day09_input));

    let day10_input = &fs::read_to_string("inputs/day10.txt")?;
    print_and_time("day 10", || day10::trailhead_scores(day10_input));

    let day11_input = &fs::read_to_string("inputs/day11.txt")?;
    print_and_time("day 11 part 1", || day11::day11(day11_input, 25));
    print_and_time("day 11 part 2",|| day11::day11(day11_input, 75));

    let day12_input = &fs::read_to_string("inputs/day12.txt")?;
    print_and_time("day 12 part 1",|| day12::fence_price(day12_input, false));
    print_and_time("day 12 part 2",|| day12::fence_price(day12_input, true));

    let day13_input = &fs::read_to_string("inputs/day13.txt")?;
    print_and_time("day 13 part 1",|| day13::min_tokens(day13_input, 0));
    print_and_time("day 13 part 2",|| day13::min_tokens(day13_input, 10000000000000));

    let day14_input = &fs::read_to_string("inputs/day14.txt")?;
    print_and_time("day 14 part 1",|| day14::safety_factor(day14_input, 100, 101, 103));
    print_and_time("day 14 part 2",|| day14::print_robots(day14_input, 101, 103));

    let day15_input = &fs::read_to_string("inputs/day15.txt")?;
    print_and_time("day 15 part 1",|| day15::gps_coordinates(day15_input));

    Ok(())
}

fn print_and_time<F: FnOnce() -> T, T: std::fmt::Debug>(text: &str, f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    println!("{}: {:?} (took {:?})", text, result, start.elapsed().unwrap());
    result
}
