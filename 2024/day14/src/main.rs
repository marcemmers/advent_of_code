use grid::{Grid, Position};
use std::cmp::Ordering::{Greater, Less};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn parse_line(input: &str) -> ((i64, i64), (i64, i64)) {
    let (pos, v) = input.split_once(' ').unwrap();

    let (pos_x, pos_y) = pos[2..].split_once(',').unwrap();
    let (v_x, v_y) = v[2..].split_once(',').unwrap();

    (
        (pos_x.parse().unwrap(), pos_y.parse().unwrap()),
        (v_x.parse().unwrap(), v_y.parse().unwrap()),
    )
}

fn solve1(input: &str, width: i64, height: i64) -> u64 {
    let lines = input.lines();

    let mut quadrants = [0, 0, 0, 0];

    let mid_x = width / 2;
    let mid_y = height / 2;

    lines.map(parse_line).for_each(|((p_x, p_y), (v_x, v_y))| {
        let end_x = (((p_x + v_x * 100) % width) + width) % width;
        let end_y = (((p_y + v_y * 100) % height) + height) % height;

        match (end_x.cmp(&mid_x), end_y.cmp(&mid_y)) {
            (Less, Less) => quadrants[0] += 1,
            (Less, Greater) => quadrants[1] += 1,
            (Greater, Less) => quadrants[2] += 1,
            (Greater, Greater) => quadrants[3] += 1,
            _ => (),
        }
    });

    quadrants.iter().product()
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    let mut robots: Vec<((i64, i64), (i64, i64))> = lines.map(parse_line).collect();

    let width = 101;
    let height = 103;

    for i in 0..100000 {
        for mid_x in 0..width {
            let set: HashSet<(i64, i64)> = HashSet::from_iter(robots.iter().map(|(p, _)| *p));

            let count = set
                .iter()
                .filter(|(p_x, p_y)| *p_x == mid_x || set.contains(&((mid_x * 2) - p_x - 1, *p_y)))
                .count();

            if count > (robots.len() as f32 * 0.5) as usize {
                let mut grid = Grid::new(width as usize, height as usize, '.');

                set.iter().for_each(|(p_x, p_y)| {
                    *grid
                        .get_mut(Position {
                            x: *p_x as i32,
                            y: *p_y as i32,
                        })
                        .unwrap() = 'X'
                });
                println!("Found potential tree at step {}", i);
                grid.print();
                return i;
            }
        }

        // Step once
        robots.iter_mut().for_each(|((p_x, p_y), (v_x, v_y))| {
            *p_x = (((*p_x + *v_x) % width) + width) % width;
            *p_y = (((*p_y + *v_y) % height) + height) % height;
        });
    }

    0
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE, 101, 103));
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
        assert_eq!(solve1(EXAMPLE, 11, 7), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 0);
    }
}
