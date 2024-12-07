use grid::{Direction, Grid, Position};
use std::{collections::HashSet, time::Instant};

fn get_char(dir: Direction) -> char {
    match dir {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

#[derive(PartialEq)]
enum Outcome {
    OutOfMap,
    Loop,
}

fn set_step(map: &mut Grid, pos: &mut Position, dir: &mut Direction) -> Option<Outcome> {
    let next = pos.step(*dir);

    if let Some(ch) = map.get_mut(next) {
        if *ch == '#' {
            *dir = dir.turn_right();
        } else if *ch == get_char(*dir) {
            return Some(Outcome::Loop);
        } else {
            *pos = next;
            *ch = get_char(*dir);
        }
    } else {
        return Some(Outcome::OutOfMap);
    }
    None
}

fn walk_path(map: &mut Grid, mut pos: Position, mut dir: Direction) -> Outcome {
    loop {
        if let Some(val) = set_step(map, &mut pos, &mut dir) {
            return val;
        }
    }
}

fn solve1(input: &str) -> u64 {
    let mut map = Grid::from_text(input);

    let pos = map.find_one('^').expect("To have start position");

    // Start with up direction
    if walk_path(&mut map, pos, Direction::Up) == Outcome::Loop {
        return 0;
    }

    map.count_filtered(|ch| ch != '#' && ch != '.') as u64
}

fn solve2(input: &str) -> u64 {
    let mut clean_map = Grid::from_text(input);

    let start = clean_map.find_one('^').expect("To have start position");

    if let Some(ch) = clean_map.get_mut(start) {
        *ch = '.';
    }

    let clean_map = clean_map;

    let mut tracker = clean_map.clone();

    let mut set: HashSet<Position> = HashSet::new();

    let mut dir = Direction::Up;
    let mut pos = start;

    loop {
        if set_step(&mut tracker, &mut pos, &mut dir) == Some(Outcome::OutOfMap) {
            break;
        }
        let mut map = clean_map.clone();

        let block = pos.step(dir);
        if let Some(ch) = map.get_mut(block) {
            *ch = '#';
        }

        if walk_path(&mut map, start, Direction::Up) == Outcome::Loop {
            set.insert(block);
        }
    }

    set.len() as u64
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
        assert_eq!(solve1(EXAMPLE), 41);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 6);
    }
}
