use std::fs;
use std::time::Instant;

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn solve1(filename: &str) -> i32 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let chars = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
        let mut combined = chars.first().unwrap().to_string();
        combined.push(*chars.last().unwrap());
        let number = combined.parse::<i32>().expect("Should have been a number");
        // println!("Number: {number}");
        sum += number;
    }

    sum
}

fn solve2(filename: &str) -> i32 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let mut replaced_line = line.to_string();

        let values = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        loop {
            if replaced_line.chars().next().unwrap().is_numeric() {
                break;
            }
            let mut found = false;

            for (i, value) in values.iter().enumerate() {
                if replaced_line.starts_with(value) {
                    replaced_line.insert_str(0, (i + 1).to_string().as_str());
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            replaced_line.remove(0);
        }

        loop {
            if replaced_line.chars().last().unwrap().is_numeric() {
                break;
            }
            let mut found = false;

            for (i, value) in values.iter().enumerate() {
                if replaced_line.ends_with(value) {
                    replaced_line.push_str((i + 1).to_string().as_str());
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            replaced_line.pop();
        }

        let chars = replaced_line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<_>>();
        let mut combined = chars.first().unwrap().to_string();
        combined.push(*chars.last().unwrap());
        let number = combined.parse::<i32>().expect("Should have been a number");
        // println!("Number: {number}");
        sum += number;
    }

    sum
}

fn main() {
    let start = Instant::now();
    let sum = solve1(PUZZLE_FILENAME);
    println!("Total: {sum}");
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    let sum = solve2(PUZZLE_FILENAME);
    println!("Total: {sum}");
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &str = "./src/example.txt";
    const EXAMPLE2_FILENAME: &str = "./src/example2.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 142);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE2_FILENAME), 281);
    }
}
