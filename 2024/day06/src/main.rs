use std::time::Instant;

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

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    let mut chars: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut pos = find_start(&chars);
    // println!("Starting at {},{}", pos.x, pos.y);

    // Start with up direction
    let mut dir = Dir::Up;

    loop {
        let next = pos.step(dir);

        if let Some(ch) = chars
            .get_mut(next.y as usize)
            .and_then(|line| line.get_mut(next.x as usize))
        {
            if *ch == '#' {
                dir = dir.turn_right();
            } else {
                pos = next;
                *ch = dir.get_char();
            }
        } else {
            break;
        }
    }

    // chars.iter().for_each(|line| {
    //     println!("{}", line.iter().collect::<String>());
    // });

    chars
        .iter()
        .map(|line| line.iter().filter(|ch| **ch != '#' && **ch != '.').count() as u64)
        .sum()
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    0
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
        assert_eq!(solve2(EXAMPLE), 0);
    }
}
