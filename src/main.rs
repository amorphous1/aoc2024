use std::fs;

fn main() -> Result<(), std::io::Error> {
    // let day01_input = &fs::read_to_string("inputs/day01.input")?;
    // println!("{}", aoc2024::day01a(day01_input));
    // println!("{}", aoc2024::day01b(day01_input));

    let day02_input = &fs::read_to_string("inputs/day02.input")?;
    println!("{}", aoc2024::day02a(day02_input));

    Ok(())
}
