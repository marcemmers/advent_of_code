use std::fs;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn get_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn collect_points(input: &Vec<&str>) -> Vec<Point> {
    let mut result = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                result.push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    return result;
}

fn solve(filename: &str, empty_space: i64) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines: Vec<&str> = input.lines().collect();

    let mut galaxies = collect_points(&lines);

    let increment = empty_space - 1;

    for (cur_y, line) in lines.iter().enumerate().rev() {
        if !line.contains("#") {
            galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.y > cur_y as i64 {
                    galaxy.y += increment
                }
            });
        }
    }

    let lines_it: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();

    for idx in (0..lines[0].len()).rev() {
        if lines_it.iter().all(|x| x[idx] == '.') {
            galaxies.iter_mut().for_each(|galaxy| {
                if galaxy.x > idx as i64 {
                    galaxy.x += increment
                }
            });
        }
    }

    let sum: i64 = galaxies
        .iter()
        .scan(galaxies.iter(), |it, p1| {
            it.next();
            Some(it.clone().fold(0, |acc, p2| acc + p1.get_distance(&p2)))
        })
        .sum();

    return sum as u64;
}

const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve(PUZZLE_FILENAME, 2));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve(PUZZLE_FILENAME, 1_000_000));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &'static str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve(EXAMPLE_FILENAME, 2), 374);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(EXAMPLE_FILENAME, 10), 1030);
        assert_eq!(solve(EXAMPLE_FILENAME, 100), 8410);
    }
}