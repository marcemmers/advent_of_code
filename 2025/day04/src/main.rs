use std::time::Instant;

use grid::{Distance, Grid};

fn solve1(input: &str) -> u64 {
    let grid = Grid::from_text(input);

    let mut count = 0;

    for (pos, item) in grid.iter() {
        if item != '@' {
            continue;
        }
        let mut rolls = 0;
        for dir in Distance::eight_directions() {
            if let Some('@') = grid.get(pos + *dir) {
                rolls += 1;
            }
        }
        if rolls < 4 {
            count += 1;
        }
    }

    count
}

fn solve2(input: &str) -> u64 {
    let mut grid = Grid::from_text(input);
    let mut next_grid = grid.clone();

    let mut count = 0;

    loop {
        let mut sub_count = 0;

        for (pos, item) in grid.iter() {
            if item != '@' {
                continue;
            }
            let mut rolls = 0;
            for dir in Distance::eight_directions() {
                if let Some('@') = grid.get(pos + *dir) {
                    rolls += 1;
                }
            }
            if rolls < 4 {
                sub_count += 1;
                *next_grid.get_mut(pos).unwrap() = '.';
            }
        }

        count += sub_count;
        if sub_count == 0 {
            break;
        }
        grid = next_grid.clone();
    }

    count
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
        assert_eq!(solve1(EXAMPLE), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 43);
    }
}
