use std::time::Instant;
use std::fs;

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    return 0;
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
    solve1(PUZZLE_FILENAME);
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    solve2(PUZZLE_FILENAME);
    println!("Solved 2 in {:?}", start.elapsed());
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &'static str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}