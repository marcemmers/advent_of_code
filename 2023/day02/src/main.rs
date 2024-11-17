use std::collections::HashMap;
use std::fs;
use std::time::Instant;
#[macro_use]
extern crate scan_fmt;

const EXAMPLE_FILENAME: &str = "./src/example.txt";
const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn solve1(filename: &str) {
    let limit = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let mut split1 = line.split(':');
        let game_number =
            scan_fmt!(split1.next().unwrap(), "Game {d}", i32).expect("Could not parse game");
        let split2 = split1.next().unwrap().split(';');
        let mut game_valid = true;
        for game in split2 {
            let colors = game.split(',').map(|x| x.trim());
            for color in colors {
                let (number, cube_color) =
                    scan_fmt!(color, "{d} {}", i32, String).expect("Could not parse cube");
                if let Some(max) = limit.get(cube_color.as_str()) {
                    if number > *max {
                        game_valid = false;
                    }
                } else {
                    game_valid = false;
                }
            }
        }
        if game_valid {
            sum += game_number;
        }
    }

    println!("Total: {sum}");
}

fn solve2(filename: &str) {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let mut split1 = line.split(':');
        let _game_number =
            scan_fmt!(split1.next().unwrap(), "Game {d}", i32).expect("Could not parse game");
        let split2 = split1.next().unwrap().split(';');
        let mut minimal: HashMap<&str, i32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for game in split2 {
            let colors = game.split(',').map(|x| x.trim());
            for color in colors {
                let (number, cube_color) =
                    scan_fmt!(color, "{d} {}", i32, String).expect("Could not parse cube");
                if let Some(max) = minimal.get_mut(cube_color.as_str()) {
                    if number > *max {
                        *max = number;
                    }
                }
            }
        }
        let mut multiple = 1i32;
        for (_, v) in minimal {
            multiple *= v;
        }
        sum += multiple;
    }
    println!("Total: {sum}");
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
