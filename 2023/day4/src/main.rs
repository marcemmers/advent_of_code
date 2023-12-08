use std::fs;
use std::time::Instant;

const EXAMPLE_FILENAME: &'static str = "./src/example.txt";
const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

fn solve1(filename: &str) {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let mut card = line.split(':').last().unwrap().split('|');
        let input = card.next().unwrap().split(' ').filter(|x| !x.is_empty());
        let winning = card.last().unwrap().split(' ').filter(|x| !x.is_empty());
        let mut count: u32 = 0;
        for item in input {
            if winning.clone().any(|x| x == item) {
                count += 1;
            }
        }
        let result = if count == 0 { 0 } else { 2i32.pow(count - 1) };
        sum += result;
        // println!("Winning: {result}");
    }
    println!("Result: {sum}");
}

fn solve2(filename: &str) {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();
    let nr_of_lines = lines.clone().count();

    let mut cards_total = vec![1 as usize; nr_of_lines];

    // let mut sum = 0;

    for (idx, line) in lines.enumerate() {
        let mut card = line.split(':').last().unwrap().split('|');
        let input = card.next().unwrap().split(' ').filter(|x| !x.is_empty());
        let winning = card.last().unwrap().split(' ').filter(|x| !x.is_empty());
        let mut count: usize = 0;
        for item in input {
            if winning.clone().any(|x| x == item) {
                count += 1;
            }
        }

        let cards = cards_total[idx];

        for i in 1..(count + 1) {
            if idx + i > cards_total.len() {
                break;
            }
            // println!("Add {cards} to idx {}", i + idx);
            cards_total[idx + i] += cards;
        }

        // println!("Winning: {result}");
    }

    let sum: usize = cards_total.iter().sum();

    println!("Result: {sum}");
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
