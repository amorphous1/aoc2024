pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.split('\n').map(|line| line.chars().collect()).collect();
    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut result = 0;
    let directions = vec![(-1,-1), (0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0)];

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            result += find_in(&grid, &xmas, x, y, &directions);
        }
    }
    return result;
}

pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.split('\n').map(|line| line.chars().collect()).collect();
    let mut result = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] == 'A' &&
                (grid[y-1][x-1] == 'M' && grid[y+1][x+1] == 'S' || grid[y-1][x-1] == 'S' && grid[y+1][x+1] == 'M') &&
                (grid[y-1][x+1] == 'M' && grid[y+1][x-1] == 'S' || grid[y-1][x+1] == 'S' && grid[y+1][x-1] == 'M') {
                result += 1;
            }
        }
    }
    return result;
}

fn find_in(grid: &Vec<Vec<char>>, term: &Vec<char>, x: i32, y: i32, directions: &Vec<(i32,i32)>) -> u32 {
    if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[y as usize].len() as i32 || grid[y as usize][x as usize] != term[0] {
        return 0;
    }
    if term.len() == 1 {
        return 1;
    }
    return directions.iter()
        .map(|dir| find_in(grid, &term[1..].to_vec(), x + dir.0, y + dir.1, &vec![*dir]))
        .sum();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part1(sample_input), 18);
        assert_eq!(part2(sample_input), 9);
    }

}