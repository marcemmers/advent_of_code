use grid::{Direction, Grid, Position};
use std::{collections::HashSet, time::Instant};

fn trailhead_score(grid: &Grid, pos: Position, set: &mut HashSet<Position>) -> u64 {
    let current_height = grid.get(pos).unwrap();
    let next_height = char::from_u32(current_height as u32 + 1).unwrap();
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let next_pos = pos.step(dir);
        if grid.get(next_pos) == Some(next_height) {
            if next_height == '9' {
                let _ = set.insert(next_pos);
            } else {
                trailhead_score(grid, next_pos, set);
            }
        }
    }
    set.len() as u64
}

fn trailhead_score2(grid: &Grid, pos: Position) -> u64 {
    let current_height = grid.get(pos).unwrap();
    let next_height = char::from_u32(current_height as u32 + 1).unwrap();
    let mut sum = 0;
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let next_pos = pos.step(dir);
        if grid.get(next_pos) == Some(next_height) {
            if next_height == '9' {
                sum += 1;
            } else {
                sum += trailhead_score2(grid, next_pos);
            }
        }
    }
    sum
}

fn solve1(input: &str) -> u64 {
    let grid = Grid::from_text(input);

    let starting_points: Vec<Position> = grid
        .iter()
        .filter_map(|(pos, ch)| if ch == '0' { Some(pos) } else { None })
        .collect();

    starting_points
        .iter()
        .map(|start| trailhead_score(&grid, *start, &mut HashSet::new()))
        .sum()
}

fn solve2(input: &str) -> u64 {
    let grid = Grid::from_text(input);

    let starting_points: Vec<Position> = grid
        .iter()
        .filter_map(|(pos, ch)| if ch == '0' { Some(pos) } else { None })
        .collect();

    starting_points
        .iter()
        .map(|start| trailhead_score2(&grid, *start))
        .sum()
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

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 36);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 81);
    }
}
