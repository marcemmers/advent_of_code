use std::time::Instant;

use astar::calculate_path;
use grid::{Grid, Position};

const EMPTY: char = '.';

fn parse_line(input: &str) -> Position {
    let (x, y) = input.split_once(',').unwrap();
    Position::new(x.parse().unwrap(), y.parse().unwrap())
}

fn parse_input(input: &str) -> Vec<Position> {
    let lines = input.lines();
    lines.map(parse_line).collect()
}

fn solve1(input: &str, mut grid: Grid, steps: usize) -> u64 {
    let positions = parse_input(input);

    for pos in positions.iter().take(steps) {
        *grid.get_mut(*pos).unwrap() = '#';
    }

    let path = calculate_path(
        &grid,
        Position::new(0, 0),
        Position::new(grid.width() as i32 - 1, grid.height() as i32 - 1),
        |grid, _, pos| grid.get(pos) == Some(EMPTY),
    )
    .unwrap();

    for pos in path.iter() {
        *grid.get_mut(*pos).unwrap() = 'O';
    }

    grid.print();

    path.len() as u64 - 1
}

fn solve2(input: &str, mut grid: Grid) -> u64 {
    let positions = parse_input(input);

    for pos in positions.iter() {
        *grid.get_mut(*pos).unwrap() = '#';
        let path = calculate_path(
            &grid,
            Position::new(0, 0),
            Position::new(grid.width() as i32 - 1, grid.height() as i32 - 1),
            |grid, _, pos| grid.get(pos) == Some(EMPTY),
        );

        if path.is_none() {
            println!("pos: {pos:?}");
            break;
        }
    }

    0
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    let grid = Grid::new(71, 71, EMPTY);
    println!("Result of 1: {}", solve1(PUZZLE, grid, 1024));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    let grid = Grid::new(71, 71, EMPTY);
    println!("Result of 2: {}", solve2(PUZZLE, grid));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test1() {
        let grid = Grid::new(7, 7, EMPTY);
        assert_eq!(solve1(EXAMPLE, grid, 12), 22);
    }

    #[test]
    fn test2() {
        let grid = Grid::new(7, 7, EMPTY);
        assert_eq!(solve2(EXAMPLE, grid), 0);
    }
}
