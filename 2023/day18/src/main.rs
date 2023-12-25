use std::fs;
use std::time::Instant;

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    fn from_hex(hex: &str) -> Self {
        assert_eq!(hex.len(), 6);
        RGB {
            r: u8::from_str_radix(&hex[0..2], 16).unwrap(),
            g: u8::from_str_radix(&hex[2..4], 16).unwrap(),
            b: u8::from_str_radix(&hex[4..6], 16).unwrap(),
        }
    }
}

struct Instruction {
    direction: char,
    steps: i32,
    color: RGB,
}

impl Instruction {
    fn new_from_line(line: &str) -> Self {
        let mut split = line.split(' ');
        Instruction {
            direction: split.next().unwrap().chars().nth(0).unwrap(),
            steps: split.next().unwrap().parse().unwrap(),
            color: RGB::from_hex(
                split
                    .next()
                    .unwrap()
                    .strip_prefix("(#")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap(),
            ),
        }
    }
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::new_from_line(line))
        .collect();

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut y = 0;
    let mut x = 0;

    for instr in instructions.iter() {
        match instr.direction {
            'R' => x += instr.steps,
            'L' => x -= instr.steps,
            'U' => y += instr.steps,
            'D' => y -= instr.steps,
            _ => (),
        }
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    println!("x min: {x_min}, max: {x_max}");
    println!("y min: {y_min}, max: {y_max}");

    let mut grid = vec![vec!['.'; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize];

    let mut x = -x_min as usize;
    let mut y = -y_min as usize;

    grid[y][x] = '#';

    for instr in instructions.iter() {
        for _ in 0..instr.steps {
            grid[y][x] = '#';
            match instr.direction {
                'R' => x += 1,
                'L' => x -= 1,
                'U' => y += 1,
                'D' => y -= 1,
                _ => (),
            }
        }
    }

    println!("Grid:");
    for line in grid.iter() {
        println!("{}", line.iter().collect::<String>())
    }

    // skip the last line to keep in bound. We don't need to fill anything there anyway
    for y in 0..(grid.len() - 1) {
        let mut inside = false;
        let mut detected = 0;
        let mut bottom = false;
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                if detected == 0 {
                    bottom = grid[y + 1][x] == '#';
                }
                detected += 1;
            } else {
                if detected == 1 {
                    inside = !inside;
                } else if detected > 1 {
                    if bottom != (grid[y + 1][x - 1] == '#') {
                        inside = !inside;
                    }
                }
                detected = 0;
            }
            if inside {
                grid[y][x] = '#';
            }
        }
    }

    let result = grid
        .iter()
        .map(|line| line.iter().filter(|ch| **ch == '#').count() as u64)
        .sum();

    println!("Grid:");
    for line in grid.iter() {
        println!("{}", line.iter().collect::<String>())
    }

    return result;
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    return 0;
}

const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE_FILENAME));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE_FILENAME));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &'static str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 62);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}
