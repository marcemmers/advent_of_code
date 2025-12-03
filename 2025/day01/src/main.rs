use std::time::Instant;

enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn from_line(line: &str) -> Self {
        let (dir, rest) = line.split_at(1);
        let rotation = rest.parse().unwrap();
        match dir {
            "L" => Self::Left(rotation),
            "R" => Self::Right(rotation),
            _ => panic!("Should never have this option"),
        }
    }
}

fn solve1(input: &str) -> u64 {
    let rotations = input.lines().map(Rotation::from_line);

    let mut pos: i32 = 50;
    let mut zero_count = 0;

    for rot in rotations {
        pos = match rot {
            Rotation::Left(rot) => pos - rot,
            Rotation::Right(rot) => pos + rot,
        };

        pos = (pos + 100) % 100;
        if pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solve2(input: &str) -> u64 {
    let rotations = input.lines().map(Rotation::from_line);

    let mut pos: i32 = 50;
    let mut zero_count = 0;

    for rot in rotations {
        pos = match rot {
            Rotation::Left(rot) if pos == 0 => pos - rot + 100, // Extra 100 because otherwise it will always underflow and count an extra zero
            Rotation::Left(rot) => pos - rot,
            Rotation::Right(rot) => pos + rot,
        };

        if pos == 0 {
            zero_count += 1;
        } else {
            zero_count += (pos / 100).abs();
            if pos < 0 {
                zero_count += 1;
            }
            pos = pos.rem_euclid(100);
        }
    }

    zero_count as u64
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
        assert_eq!(solve1(EXAMPLE), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 6);
    }

    #[test]
    fn test3() {
        assert_eq!(solve2("L1000"), 10);
        assert_eq!(solve2("R1000"), 10);
        assert_eq!(solve2("L500\nR500"), 10);
        assert_eq!(solve2("L50"), 1);
        assert_eq!(solve2("L450"), 5);
        assert_eq!(solve2("L450\nR550"), 10);
    }
}
