use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn transpose(input: &[String]) -> Vec<String> {
    let len = input[0].len();
    input
        .iter()
        .fold(vec![String::new(); len], |mut acc, line| {
            line.chars().enumerate().for_each(|(i, ch)| acc[i].push(ch));
            acc
        })
}

fn rotate(input: &[String]) -> Vec<String> {
    let len = input[0].len();
    input
        .iter()
        .fold(vec![String::new(); len], |mut acc, line| {
            line.chars()
                .rev()
                .enumerate()
                .for_each(|(i, ch)| acc[i].push(ch));
            acc
        })
}

fn move_to_front(line: &str) -> String {
    let mut result = String::new(); //".".repeat(line.len());

    let mut empty_space = 0;

    for ch in line.chars() {
        match ch {
            'O' => result.push('O'),
            '#' => {
                result.push_str(".".repeat(empty_space).as_str());
                result.push('#');
                empty_space = 0;
            }
            _ => empty_space += 1,
        }
    }

    if empty_space != 0 {
        result.push_str(".".repeat(empty_space).as_str());
    }

    result
}

fn calculate_load(input: &[String]) -> u64 {
    input.iter().fold(0, |acc, line| {
        acc + line.chars().rev().enumerate().fold(
            0,
            |acc, (idx, ch)| if ch == 'O' { acc + idx + 1 } else { acc },
        )
    }) as u64
}

#[allow(unused)]
fn print_grid(input: &[String]) {
    println!("Grid:");

    for line in transpose(input) {
        println!("{line}");
    }
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = transpose(
        &input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );

    let moved_lines: Vec<String> = lines
        .iter()
        .map(|line| move_to_front(line.as_str()))
        .collect();

    // print_grid(&lines);
    // print_grid(&moved_lines);

    calculate_load(&moved_lines)
}

fn move_all(input: &[String]) -> Vec<String> {
    input
        .iter()
        .map(|line| move_to_front(line.as_str()))
        .collect()
}

fn perform_full_rotation(input: &[String]) -> Vec<String> {
    // North
    let step = rotate(&move_all(input));

    // West
    let step = rotate(&move_all(&step));

    // South
    let step = rotate(&move_all(&step));

    // East
    rotate(&move_all(&step))
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = transpose(
        &input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );

    let mut map: HashMap<Vec<String>, i32> = HashMap::new();

    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut found_state: Vec<String> = Vec::new();

    let _ = (0..1_000_000_000).try_fold(lines, |state, idx| {
        // println!("Rotation: {idx}");
        let new_state = perform_full_rotation(&state);
        if let Some(found) = map.get(&new_state) {
            println!("Found match: {found}, current: {idx}");
            idx1 = *found;
            idx2 = idx;
            found_state = state.clone();
            return None;
        }
        map.insert(new_state.clone(), idx);
        Some(new_state)
    });

    let diff = idx2 - idx1;

    let remainder = (1_000_000_000 - idx2) % diff;
    println!("Remainder: {remainder}");

    let final_state = (0..remainder).fold(found_state, |state, _| perform_full_rotation(&state));

    calculate_load(&final_state)
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
        assert_eq!(solve1(EXAMPLE_FILENAME), 136);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 64);
    }
}
