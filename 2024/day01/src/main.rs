use std::fs;
use std::time::Instant;

fn parse_line(line: &str) -> (u64, u64) {
    let line: Vec<u64> = line
        .split_whitespace()
        .map(|val| val.parse::<u64>().expect("Should be able to parse"))
        .collect();
    (line[0], line[1])
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();
    let (mut list_a, mut list_b): (Vec<u64>, Vec<u64>) = lines.map(parse_line).collect();

    list_a.sort();
    list_b.sort();

    list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum()
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();
    let (list_a, list_b): (Vec<u64>, Vec<u64>) = lines.map(parse_line).collect();

    list_a
        .iter()
        .map(|a| list_b.iter().filter(|b| a == *b).count() as u64 * a)
        .sum()
}

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

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

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 31);
    }
}
