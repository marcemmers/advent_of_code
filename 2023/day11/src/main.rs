use std::collections::HashSet;
use std::time::Instant;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}


fn collect_points(input: &Vec<String>) -> HashSet<Point> {
    let mut result = HashSet::new();

    for (y, line) in input.iter().rev().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                result.insert(Point{x: x as i32, y: y as i32});
            }
        }
    }

    return result;
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines: Vec<&str> = input.lines().collect();

    let mut puzzle: Vec<String> = Vec::new();

    for line in lines.iter() {
        if line.contains("#") {
            puzzle.push(line.to_string());
        } else {
            puzzle.push(line.to_string());
            puzzle.push(line.to_string());
        }
    }

    let lines_it = lines.iter().map(|x| x.chars().collect::<Vec<char>>());

    for idx in (0..lines[0].len()).rev() {
        if lines_it.clone().all(|x| x[idx] == '.') {
            for line in puzzle.iter_mut() {
                line.insert(idx, '.');
            }
        }
    }

    for line in puzzle.iter() {
        println!("{line}");
    }

    let set = collect_points(&puzzle);

    let mut set_it = set.iter();
    set_it.next();
    let mut sum = 0;

    set.iter().for_each(|p1| {
        sum += set_it.clone().fold(0, |acc, p2| acc + p1.get_distance(&p2));
        set_it.next();
    });

    return sum as u64;
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
        assert_eq!(solve1(EXAMPLE_FILENAME), 374);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}