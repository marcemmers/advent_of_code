use std::collections::HashMap;
use std::fs;
use std::str::{Chars, Lines};
use std::time::Instant;

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn calculate_steps(
    map: &HashMap<String, (String, String)>,
    directions: Chars,
    start: String,
    end: fn(&String) -> bool,
) -> usize {
    let mut pos = start;
    let step = |cur_pos: &String, dir: char| -> String {
        let item = map.get(cur_pos.as_str()).unwrap();
        if dir == 'L' {
            item.0.clone()
        } else {
            item.1.clone()
        }
    };

    let steps = directions
        .cycle()
        .take_while(|dir| {
            pos = step(&pos, *dir);
            !end(&pos)
        })
        .count();

    steps + 1
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn generate_map(lines: Lines) -> HashMap<String, (String, String)> {
    lines
        .skip(2)
        .map(|line| {
            (
                line[0..3].to_string(),
                (line[7..10].to_string(), line[12..15].to_string()),
            )
        })
        .collect()
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();
    let map = generate_map(lines.clone());
    let directions = lines.clone().next().unwrap().chars();

    let steps = calculate_steps(&map, directions, "AAA".to_string(), |pos: &String| {
        pos.eq("ZZZ")
    });

    // println!("Map: {:?}", map);
    println!("Steps: {steps}");
    steps as u64
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();
    let map = generate_map(lines.clone());
    let directions = lines.clone().next().unwrap().chars();

    let positions: Vec<&str> = map
        .iter()
        .map(|(start, (_, _))| start.as_str())
        .filter(|start| start.ends_with("A"))
        .collect();

    let results = positions.iter().map(|pos| {
        calculate_steps(&map, directions.clone(), pos.to_string(), |pos: &String| {
            pos.ends_with("Z")
        })
    });

    let steps = results.fold(1usize, lcm);

    // println!("Map: {:?}", map);
    println!("Steps: {steps}");
    steps as u64
}

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

    const EXAMPLE_FILENAME: &str = "./src/example.txt";
    const EXAMPLE2_FILENAME: &str = "./src/example2.txt";

    #[test]
    fn test1() {
        let result = solve1(EXAMPLE_FILENAME);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let result = solve2(EXAMPLE2_FILENAME);
        assert_eq!(result, 6);
    }
}
