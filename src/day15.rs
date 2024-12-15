pub fn gps_coordinates(input: &str) -> usize {
    let (map, moves) = input.split_at(input.find("\n\n").unwrap());
    let mut map: Vec<Vec<char>> = map.split("\n").map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = moves.replace('\n', "").chars().collect();
    let mut pos = find_robot(&map);
    for mov in moves {
        pos = move_robot(mov, &mut map, pos);
    }
    let mut result = 0;
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, ch)| {
            if *ch == 'O' {
                result += y * 100 + x;
            }
        });
    });
    return result;
}

fn move_robot(mov: char, map: &mut Vec<Vec<char>>, (x, y): (i16, i16)) -> (i16, i16) {
    let (dx, dy): (i16, i16) = match mov  {
        '^' => { (0, -1) },
        'v' => { (0, 1) },
        '<' => { (-1, 0) },
        '>' => { (1, 0) },
        _ => { panic!("unexpected movement '{}'", mov) },
    };
    let (mut tx, mut ty) = (x + dx, y + dy);
    while char_at(tx, ty, map) == 'O' {
        (tx, ty) = (tx + dx, ty + dy);
    }
    if char_at(tx, ty, map) == '#' { // no empty space after boxes, nothing moves
        return (x, y);
    }
    map[ty as usize][tx as usize] = 'O';
    map[(y + dy) as usize][(x + dx) as usize] = '@';
    map[y as usize][x as usize] = '.';
    return (x + dx, y + dy);
}

fn char_at(x: i16, y: i16, map: &Vec<Vec<char>>) -> char {
    return map[y as usize][x as usize];
}

fn find_robot(map: &Vec<Vec<char>>) -> (i16, i16) {
    for y in 0.. map.len() {
        let row = map.get(y).unwrap();
        for x in 0..row.len() {
            if *row.get(x).unwrap() == '@' {
                return (x as i16, y as i16);
            }
        }
    }
    panic!("cannot find robot (@) in {:?}", map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let sample_input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(gps_coordinates(sample_input), 2028);
    }

    #[test]
    fn large_sample() {
        let sample_input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(gps_coordinates(sample_input), 10092);
    }
}