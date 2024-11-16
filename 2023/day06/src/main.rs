use std::fs;
use std::time::Instant;

const EXAMPLE_FILENAME: &'static str = "./src/example.txt";
const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

// fn calculate_distances(time: u64, min_distance: u64) -> u64 {
//     return (0..=time)
//         .map(|i| (time - i) * i)
//         .filter(|x| *x > min_distance)
//         .count() as u64;
// }

fn calculate_distances_math(time: u64, min_distance: u64) -> u64 {
    let a = -1f64;
    let b = time as f64;
    let c = -(min_distance as f64 + 0.001);

    let d = b.powf(2.0) - (4.0 * a * c);

    let s1 = (-b + d.sqrt()) / (2.0 * a);
    let s2 = (-b - d.sqrt()) / (2.0 * a);

    let s1 = s1.ceil();
    let s2 = s2.floor();

    return (s2 - s1) as u64 + 1;
}

fn solve1(filename: &str) {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let mut lines = input.lines();
    let convert = |line: &str| -> Vec<u64> {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .collect()
    };
    let times = convert(lines.next().unwrap());
    let distance = convert(lines.next().unwrap());

    println!("Times: {:?}", times);
    println!("Distance: {:?}", distance);

    let iter = times.iter().zip(distance.iter());

    let mult = iter.fold(1u64, |mult, item| {
        mult * calculate_distances_math(*item.0, *item.1)
    });
    println!("Mult: {mult}");
}

fn solve2(filename: &str) {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let mut lines = input.lines();
    let convert = |line: &str| -> u64 {
        line.split_once(':')
            .unwrap()
            .1
            .replace(" ", "")
            .parse::<u64>()
            .unwrap()
    };
    let time = convert(lines.next().unwrap());
    let distance = convert(lines.next().unwrap());

    println!("Times: {:?}", time);
    println!("Distance: {:?}", distance);

    let mult = calculate_distances_math(time, distance);

    println!("Mult: {mult}");
}

fn main() {
    let start = Instant::now();
    solve1(EXAMPLE_FILENAME);
    solve1(PUZZLE_FILENAME);

    println!("Solved 1 in {:?}\n\n", start.elapsed());
    let start = Instant::now();

    solve2(EXAMPLE_FILENAME);
    solve2(PUZZLE_FILENAME);
    println!("Solved 2 in {:?}", start.elapsed());
}
