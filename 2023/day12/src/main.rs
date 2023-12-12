use std::fs;
use std::time::Instant;

fn equal_lengths(map: &str, numbers: &[u32]) -> bool {
    let map = map
        .split('.')
        .filter_map(|x| if x.is_empty() { None } else { Some(x.len()) });

    map.clone().count() == numbers.len() && map.zip(numbers).all(|(a, b)| a as u32 == *b)
}

fn try_arrangements(map: &str, numbers: &[u32], questions_left: usize) -> u64 {
    if questions_left == 0 {
        return if equal_lengths(map, numbers) { 1 } else { 0 };
    }

    return try_arrangements(
        map.replacen("?", ".", 1).as_str(),
        &numbers,
        questions_left - 1,
    ) + try_arrangements(
        map.replacen("?", "#", 1).as_str(),
        &numbers,
        questions_left - 1,
    );
}

fn calculate_arrangements(line: &str) -> u64 {
    let (map, numbers) = line.split_once(' ').unwrap();

    let numbers: Vec<u32> = numbers
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    return try_arrangements(
        map,
        numbers.as_slice(),
        map.chars().filter(|x| *x == '?').count(),
    );
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let result = lines.map(|line| calculate_arrangements(line)).sum();

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
        assert_eq!(solve1(EXAMPLE_FILENAME), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}
