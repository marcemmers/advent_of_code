use std::fs;
use std::time::Instant;

struct Instruction {
    direction: char,
    steps: i64,
}

impl Instruction {
    fn new_from_line(line: &str, use_rgb: bool) -> Self {
        if use_rgb {
            let split = line.split_once('#').unwrap().1.strip_suffix(')').unwrap();
            Instruction {
                direction: match split.chars().nth(5).unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => 'U',
                },
                steps: i64::from_str_radix(&split[0..5], 16).unwrap(),
            }
        } else {
            let mut split = line.split(' ');
            Instruction {
                direction: split.next().unwrap().chars().nth(0).unwrap(),
                steps: split.next().unwrap().parse().unwrap(),
            }
        }
    }
}

fn solve(filename: &str, part2: bool) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::new_from_line(line, part2))
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

    let mut x = -x_min;
    let mut y = -y_min;
    let mut sum = 0;

    for instr in instructions.iter() {
        let prev_x = x;
        let prev_y = y;
        match instr.direction {
            'R' => x += instr.steps,
            'L' => x -= instr.steps,
            'U' => y += instr.steps,
            'D' => y -= instr.steps,
            _ => (),
        }

        sum += (prev_x * y) - (x * prev_y);
    }

    sum = sum.abs() / 2;

    sum += instructions.iter().map(|i| i.steps).sum::<i64>() / 2; // add the route walked
    sum += 1; // Add 1 for some reason

    println!("Sum: {sum}");

    sum as u64
}

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve(PUZZLE_FILENAME, false));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve(PUZZLE_FILENAME, true));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve(EXAMPLE_FILENAME, false), 62);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(EXAMPLE_FILENAME, true), 952408144115);
    }
}
