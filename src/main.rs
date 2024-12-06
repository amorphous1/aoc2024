use std::fs;

fn main() -> Result<(), std::io::Error> {
    // let day01_input = &fs::read_to_string("inputs/day01.input")?;
    // println!("{}", aoc2024::day01a(day01_input));
    // println!("{}", aoc2024::day01b(day01_input));

    // let day02_input = &fs::read_to_string("inputs/day02.input")?;
    // println!("{}", aoc2024::day02(day02_input, false));
    // println!("{}", aoc2024::day02(day02_input, true));

    // let day03_input = &fs::read_to_string("inputs/day03.input")?;
    // println!("{}", aoc2024::day03a(day03_input));
    // println!("{}", aoc2024::day03b(day03_input));

    // let day04_input = &fs::read_to_string("inputs/day04.input")?;
    // println!("{}", aoc2024::day04a(day04_input));
    // println!("{}", aoc2024::day04b(day04_input));

    let day05_input = &fs::read_to_string("inputs/day05.input")?;
    println!("{}", aoc2024::day05a(day05_input));

    Ok(())
}
