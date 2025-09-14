use std::{collections::HashMap, time::Instant};

use grid::{Direction, Distance, Grid};

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';
const EMPTY: char = '.';

fn solve1(input: &str) -> usize {
    let mut grid = Grid::from_text(input);
    let start = grid.find_one(START).unwrap();
    let end = grid.find_one(END).unwrap();

    grid.replace_all(END, EMPTY);
    grid.replace_all(START, EMPTY);

    let mut positions: HashMap<grid::Position, i32> = HashMap::new();

    positions.insert(start, 0);

    let mut count = 1;
    let mut pos = start;
    while pos != end {
        for dir in Direction::all_directions() {
            let next = pos + dir;
            if grid.get(next) == Some(EMPTY) {
                *grid.get_mut(pos).unwrap() = WALL;
                positions.insert(next, count);
                count += 1;
                pos = next;
            }
        }
    }

    let mut grid = Grid::from_text(input);
    grid.replace_all(END, EMPTY);
    grid.replace_all(START, EMPTY);

    let mut times_saved: HashMap<i32, usize> = HashMap::new();

    for (item, ch) in grid.iter() {
        if ch != WALL {
            continue;
        }

        let time_saved = match (
            grid.get(item + Direction::Up),
            grid.get(item + Direction::Down),
            grid.get(item + Direction::Left),
            grid.get(item + Direction::Right),
        ) {
            (Some(EMPTY), Some(EMPTY), _, _) => {
                (positions.get(&(item + Direction::Up)).unwrap()
                    - positions.get(&(item + Direction::Down)).unwrap())
                .abs()
                    - 2
            }
            (_, _, Some(EMPTY), Some(EMPTY)) => {
                (positions.get(&(item + Direction::Left)).unwrap()
                    - positions.get(&(item + Direction::Right)).unwrap())
                .abs()
                    - 2
            }
            _ => continue,
        };

        *times_saved.entry(time_saved).or_default() += 1;
    }

    // let mut times_saved_vec: Vec<_> = times_saved.iter().collect();
    // times_saved_vec.sort_by_key(|v| v.0);

    // times_saved_vec
    //     .iter()
    //     .for_each(|(time, count)| println!("Time {time}: {count}"));

    times_saved
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, nr)| nr)
        .sum()
}

fn solve2(input: &str) -> usize {
    let mut grid = Grid::from_text(input);
    let start = grid.find_one(START).unwrap();
    let end = grid.find_one(END).unwrap();

    grid.replace_all(END, EMPTY);
    grid.replace_all(START, EMPTY);

    let mut positions: HashMap<grid::Position, i32> = HashMap::new();

    positions.insert(start, 0);

    let mut count = 1;
    let mut pos = start;
    while pos != end {
        for dir in Direction::all_directions() {
            let next = pos + dir;
            if grid.get(next) == Some(EMPTY) {
                *grid.get_mut(pos).unwrap() = WALL;
                positions.insert(next, count);
                count += 1;
                pos = next;
            }
        }
    }

    let mut grid = Grid::from_text(input);
    grid.replace_all(END, EMPTY);
    grid.replace_all(START, EMPTY);

    let mut times_saved: HashMap<i32, usize> = HashMap::new();

    for (item, time) in positions.iter() {
        for x in -20i32..=20 {
            for y in -20i32..=20 {
                if x.abs() + y.abs() <= 20
                    && (x != 0 || y != 0)
                    && grid.get(*item + Distance { x, y }) == Some(EMPTY)
                {
                    let time_saved = (time - positions.get(&(*item + Distance { x, y })).unwrap())
                        .abs()
                        - x.abs()
                        - y.abs();

                    *times_saved.entry(time_saved).or_default() += 1;
                }
            }
        }
    }

    // let mut times_saved_vec: Vec<_> = times_saved.iter().collect();
    // times_saved_vec.sort_by_key(|v| v.0);

    // times_saved_vec
    //     .iter()
    //     .for_each(|(time, count)| println!("Time {time}: {count}"));

    times_saved
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, nr)| nr / 2)
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
        assert_eq!(solve1(EXAMPLE), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 0);
    }
}
