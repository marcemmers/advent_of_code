use std::{collections::HashSet, time::Instant};

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn get_char(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn find_start(map: &[Vec<char>]) -> Position {
    map.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, ch)| if *ch == '^' { Some(x) } else { None })
                .map(|x| Position {
                    x: x as i32,
                    y: y as i32,
                })
        })
        .expect("Should have some")
}

#[derive(PartialEq)]
enum Outcome {
    OutOfMap,
    Loop,
}

fn set_step(map: &mut [Vec<char>], pos: &mut Position, dir: &mut Dir) -> Option<Outcome> {
    let next = pos.step(*dir);

    if let Some(ch) = map
        .get_mut(next.y as usize)
        .and_then(|line| line.get_mut(next.x as usize))
    {
        if *ch == '#' {
            *dir = dir.turn_right();
        } else if *ch == dir.get_char() {
            return Some(Outcome::Loop);
        } else {
            *pos = next;
            *ch = dir.get_char();
        }
    } else {
        return Some(Outcome::OutOfMap);
    }
    None
}

fn walk_path(map: &mut [Vec<char>], mut pos: Position, mut dir: Dir) -> Outcome {
    loop {
        if let Some(val) = set_step(map, &mut pos, &mut dir) {
            return val;
        }
    }
}

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    let mut map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let pos = find_start(&map);

    // Start with up direction
    if walk_path(&mut map, pos, Dir::Up) == Outcome::Loop {
        return 0;
    }

    map.iter()
        .map(|line| line.iter().filter(|ch| **ch != '#' && **ch != '.').count() as u64)
        .sum()
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    let mut clean_map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let start = find_start(&clean_map);

    if let Some(ch) = clean_map
        .get_mut(start.y as usize)
        .and_then(|line| line.get_mut(start.x as usize))
    {
        *ch = '.';
    }

    let clean_map = clean_map;

    let mut tracker = clean_map.clone();

    let mut set: HashSet<Position> = HashSet::new();

    let mut dir = Dir::Up;
    let mut pos = start;

    loop {
        if set_step(&mut tracker, &mut pos, &mut dir) == Some(Outcome::OutOfMap) {
            break;
        }
        let mut map = clean_map.clone();

        let block = pos.step(dir);
        if let Some(ch) = map
            .get_mut(block.y as usize)
            .and_then(|line| line.get_mut(block.x as usize))
        {
            *ch = '#';
        }

        if walk_path(&mut map, start, Dir::Up) == Outcome::Loop {
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
