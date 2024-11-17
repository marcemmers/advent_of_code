use std::fs;
use std::str::Lines;
use std::time::Instant;

fn calculate_differences(s1: &[&str], s2: &[&str]) -> u64 {
    s1.iter()
        .rev()
        .zip(s2.iter())
        .map(|(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() as u64)
        .sum()
}

fn find_mirror_row(lines: &Vec<&str>, diff: u64) -> Option<u64> {
    for i in 1..lines.len() {
        let (p1, p2) = lines.split_at(i);

        if calculate_differences(p1, p2) == diff {
            return Some(i as u64);
        }
    }
    None
}

fn transpose(lines: Lines) -> Vec<String> {
    let len = lines.clone().next().unwrap().len();
    lines.fold(vec![String::new(); len], |mut acc, line| {
        line.chars().enumerate().for_each(|(i, ch)| acc[i].push(ch));
        acc
    })
}

fn calculate_mirror_value(block: &str, diff: u64) -> u64 {
    // Try the horizontal mirror
    let lines: Vec<&str> = block.lines().collect();
    if let Some(row) = find_mirror_row(&lines, diff) {
        return row * 100;
    }

    // Try the vertical mirror
    let lines = transpose(block.lines());
    let lines_ref: Vec<&str> = lines.iter().map(|x| x.as_str()).collect();
    if let Some(row) = find_mirror_row(&lines_ref, diff) {
        return row;
    }

    println!("Found none for:");
    println!("{block}");
    0
}

fn solve(filename: &str, diff: u64) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let blocks: Vec<&str> = input.split("\n\n").collect();

    blocks.iter().map(|x| calculate_mirror_value(x, diff)).sum()
}

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve(PUZZLE_FILENAME, 0));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve(PUZZLE_FILENAME, 1));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve(EXAMPLE_FILENAME, 0), 405);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(EXAMPLE_FILENAME, 1), 400);
    }
}
