use std::time::Instant;
use std::{fs, str::Lines};

#[derive(Eq, PartialEq, Hash, Clone)]
struct MapItem {
    src: u32,
    dst: u32,
    size: u32,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Seed {
    start: u32,
    size: u32,
}

fn parse_to_u32(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

fn generate_map(lines: &mut Lines) -> Vec<MapItem> {
    let mut result: Vec<MapItem> = Vec::new();
    loop {
        let line = lines.next();
        if line == None {
            break;
        }
        let line = line.unwrap().trim();
        if line.is_empty() {
            break;
        }

        let mut values = line.split(' ').map(parse_to_u32);
        result.push(MapItem {
            dst: values.next().unwrap(),
            src: values.next().unwrap(),
            size: values.next().unwrap(),
        });
    }
    return result;
}

fn generate_transformation(lines: Lines) -> Vec<Vec<MapItem>> {
    let mut lines = lines.clone();
    let mut transforms: Vec<Vec<MapItem>> = Vec::new();

    loop {
        let line = lines.next();
        if line == None {
            break;
        }
        let line: &str = line.unwrap().trim();
        if line.is_empty() {
            continue;
        }
        if line.contains("map:") {
            transforms.push(generate_map(&mut lines));
        }
    }
    return transforms;
}

fn perform_transform(tranform: &Vec<MapItem>, item: u32) -> u32 {
    for map in tranform {
        if item >= map.src && item < map.src + map.size {
            let dst = map.dst + (item - map.src);
            // println!("Found match: {item} -> {dst}");
            return dst;
        }
    }
    return item;
}

fn find_location(transforms: &Vec<Vec<MapItem>>, seed: u32) -> u32 {
    let result = transforms
        .iter()
        .fold(seed, |acc, e| perform_transform(e, acc));
    return result;
}

fn get_seeds(line: &str) -> Vec<u32> {
    line.replace("seeds: ", "")
        .split(' ')
        .map(parse_to_u32)
        .collect()
}

fn solve1(filename: &str) -> u32 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let mut lines = input.lines();

    let seeds = get_seeds(lines.next().unwrap());

    let transforms = generate_transformation(lines);

    let nr_of_maps = transforms.len();
    println!("Nr of maps: {nr_of_maps}");

    let results = seeds.iter().map(|seed| find_location(&transforms, *seed));

    return results.min().unwrap();
}

fn solve2(filename: &str) -> u32 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let mut lines = input.lines();

    let seeds_raw = get_seeds(lines.next().unwrap());
    let seeds: Vec<Seed> = seeds_raw
        .as_slice()
        .windows(2)
        .step_by(2)
        .map(|val| Seed {
            start: val[0],
            size: val[1],
        })
        .collect();
    println!("Seeds: {:?}", seeds);

    let transforms = generate_transformation(lines);

    let nr_of_maps = transforms.len();
    println!("Nr of maps: {nr_of_maps}");

    let mut min = u32::MAX;

    for seed in seeds {
        min = (seed.start..(seed.start + seed.size))
            .fold(min, |min, e| u32::min(min, find_location(&transforms, e)));
        println!("Found new min: {min}");
    }

    return min;
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
        assert_eq!(solve1(EXAMPLE_FILENAME), 35);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 46);
    }
}
