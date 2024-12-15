use std::{collections::HashSet, time::Instant};

use grid::{Direction, Grid, Position};

fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let (grid, directions) = input.split_once("\n\n").unwrap();

    (
        Grid::from_text(grid),
        directions
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(Direction::from_char)
            .collect(),
    )
}

fn do_move(grid: &mut Grid, pos: Position, dir: Direction) -> Position {
    let mut boxes = 0;
    let mut moved = pos.step(dir);
    while let Some(next) = grid.get(moved) {
        match next {
            '#' => return pos,
            'O' => boxes += 1,
            _ => break,
        }
        moved = moved.step(dir);
    }

    *grid.get_mut(pos).unwrap() = '.';

    let robot_pos = pos.step(dir);
    *grid.get_mut(robot_pos).unwrap() = '@';

    let mut moved = robot_pos;

    for _ in 0..boxes {
        moved = moved.step(dir);
        *grid.get_mut(moved).unwrap() = 'O';
    }

    robot_pos
}

fn calculate_gps(grid: &Grid, box_ch: char) -> u64 {
    grid.iter()
        .filter(|(_, ch)| *ch == box_ch)
        .map(|(pos, _)| pos.y as u64 * 100 + pos.x as u64)
        .sum()
}

fn solve1(input: &str) -> u64 {
    let (mut grid, directions) = parse(input);

    let mut position = grid.find_one('@').unwrap();

    for dir in directions {
        position = do_move(&mut grid, position, dir);
    }

    calculate_gps(&grid, 'O')
}

fn widen_grid(grid: &Grid) -> Grid {
    let mut new = Grid::with_capacity(grid.width() * 2, grid.height());

    grid.iter_rows()
        .zip(new.iter_rows_mut())
        .for_each(|((_, old), (_, new))| {
            for ch in old {
                match ch {
                    '#' => {
                        new.push('#');
                        new.push('#');
                    }
                    'O' => {
                        new.push('[');
                        new.push(']');
                    }
                    '@' => {
                        new.push('@');
                        new.push('.');
                    }
                    _ => {
                        new.push('.');
                        new.push('.');
                    }
                }
            }
        });
    new
}

fn do_wide_move_sideways(grid: &mut Grid, pos: Position, dir: Direction) -> Position {
    let mut boxes = 0;
    let mut moved = pos.step(dir);
    while let Some(next) = grid.get(moved) {
        match (next, dir) {
            ('#', _) => return pos,
            (']', Direction::Left) | ('[', Direction::Right) => boxes += 1,
            _ => break,
        }
        moved = moved.step(dir).step(dir);
    }

    *grid.get_mut(pos).unwrap() = '.';

    let robot_pos = pos.step(dir);
    *grid.get_mut(robot_pos).unwrap() = '@';

    let mut moved = robot_pos;

    for _ in 0..boxes {
        moved = moved.step(dir);
        match dir {
            Direction::Left => {
                *grid.get_mut(moved).unwrap() = ']';
                moved = moved.step(dir);
                *grid.get_mut(moved).unwrap() = '[';
            }
            Direction::Right => {
                *grid.get_mut(moved).unwrap() = '[';
                moved = moved.step(dir);
                *grid.get_mut(moved).unwrap() = ']';
            }
            _ => (),
        }
    }

    robot_pos
}

fn do_wide_move_up_down(grid: &mut Grid, pos: Position, dir: Direction) -> Position {
    let mut moved = pos.step(dir);

    let mut boxes_to_move: Vec<Position> = Vec::new();

    let mut x_coords: HashSet<i32> = HashSet::new();
    x_coords.insert(pos.x);

    loop {
        let mut empty = 0;
        let mut next_coords: HashSet<i32> = HashSet::new();
        for step in x_coords.iter().map(|x| Position::new(*x, moved.y)) {
            let next = grid.get(step).unwrap();
            match next {
                '#' => return pos,
                ']' => {
                    next_coords.insert(step.x);
                    next_coords.insert(step.x - 1);
                    boxes_to_move.push(step.step(Direction::Left));
                }
                '[' => {
                    next_coords.insert(step.x);
                    next_coords.insert(step.x + 1);
                    boxes_to_move.push(step);
                }
                _ => empty += 1,
            }
        }

        if empty == x_coords.len() {
            break;
        }

        x_coords = next_coords;
        moved = moved.step(dir);
    }

    *grid.get_mut(pos).unwrap() = '.';

    let robot_pos = pos.step(dir);

    while let Some(item) = boxes_to_move.pop() {
        *grid.get_mut(item).unwrap() = '.';
        *grid.get_mut(item.step(Direction::Right)).unwrap() = '.';

        let new_pos = item.step(dir);
        *grid.get_mut(new_pos).unwrap() = '[';
        *grid.get_mut(new_pos.step(Direction::Right)).unwrap() = ']';
    }

    *grid.get_mut(robot_pos).unwrap() = '@';

    robot_pos
}

fn do_wide_move(grid: &mut Grid, pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up | Direction::Down => do_wide_move_up_down(grid, pos, dir),
        Direction::Left | Direction::Right => do_wide_move_sideways(grid, pos, dir),
    }
}

fn solve2(input: &str) -> u64 {
    let (grid, directions) = parse(input);

    let mut grid = widen_grid(&grid);
    let mut position = grid.find_one('@').unwrap();

    for dir in directions {
        position = do_wide_move(&mut grid, position, dir);
    }

    calculate_gps(&grid, '[')
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");
    const EXAMPLE2: &str = include_str!("./example2.txt");
    const EXAMPLE3: &str = include_str!("./example3.txt");

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 2028);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solve1(EXAMPLE2), 10092);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE2), 9021);
    }

    #[test]
    fn test2_example3() {
        assert_eq!(solve2(EXAMPLE3), 9021);
    }
}
