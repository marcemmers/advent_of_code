use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use grid::{Direction, Grid, Position};

fn calculate_plot_cost(grid: &mut Grid, start: Position) -> u64 {
    let crop = grid.get(start).unwrap();

    let mut to_visit: VecDeque<Position> = VecDeque::new();
    let mut plot: HashSet<Position> = HashSet::new();

    to_visit.push_back(start);
    plot.insert(start);

    let mut perimiter = 0;

    while let Some(pos) = to_visit.pop_front() {
        for dir in Direction::all_directions() {
            let step = pos.step(dir);
            if grid.get(step) == Some(crop) {
                if plot.insert(step) {
                    to_visit.push_back(step);
                }
            } else {
                perimiter += 1;
            }
        }
    }

    plot.iter()
        .for_each(|pos| *grid.get_mut(*pos).unwrap() = '.');

    plot.len() as u64 * perimiter
}

fn solve1(input: &str) -> u64 {
    let mut grid = Grid::from_text(input);

    let mut sum = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Position::new(x as i32, y as i32);
            let item = grid.get(pos).unwrap();
            if item == '.' {
                continue;
            }

            sum += calculate_plot_cost(&mut grid, pos);
        }
    }

    sum
}

fn calculate_bulk_perimiter(grid: &Grid) -> u64 {
    let mut prev_right: HashSet<usize> = HashSet::new();
    let mut prev_left: HashSet<usize> = HashSet::new();
    let mut active = false;

    let mut sum = 0;

    grid.iter_rows().for_each(|(_, row)| {
        let mut right: HashSet<usize> = HashSet::new();
        let mut left: HashSet<usize> = HashSet::new();
        row.iter().enumerate().for_each(|(i, ch)| {
            if *ch == '+' {
                if !active {
                    active = true;
                    right.insert(i);
                }
            } else if active {
                active = false;
                left.insert(i);
            }
        });

        sum += right.difference(&prev_right).count();
        sum += left.difference(&prev_left).count();

        prev_right = right;
        prev_left = left;
    });

    sum as u64
}

fn calculate_plot_cost_with_bulk(grid: &mut Grid, start: Position) -> u64 {
    let crop = grid.get(start).unwrap();

    let mut to_visit: VecDeque<Position> = VecDeque::new();
    let mut plot: HashSet<Position> = HashSet::new();

    to_visit.push_back(start);
    plot.insert(start);

    while let Some(pos) = to_visit.pop_front() {
        for dir in Direction::all_directions() {
            let step = pos.step(dir);
            if grid.get(step) == Some(crop) && plot.insert(step) {
                to_visit.push_back(step);
            }
        }
    }

    plot.iter()
        .for_each(|pos| *grid.get_mut(*pos).unwrap() = '+');

    let perimiter = calculate_bulk_perimiter(grid) + calculate_bulk_perimiter(&grid.transpose());

    plot.iter()
        .for_each(|pos| *grid.get_mut(*pos).unwrap() = '.');

    plot.len() as u64 * perimiter
}

fn solve2(input: &str) -> u64 {
    let mut grid = Grid::from_text(input);

    let mut sum = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Position::new(x as i32, y as i32);
            let item = grid.get(pos).unwrap();
            if item == '.' {
                continue;
            }

            sum += calculate_plot_cost_with_bulk(&mut grid, pos);
        }
    }

    sum
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
        assert_eq!(solve1(EXAMPLE), 1930);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 1206);
    }
}
