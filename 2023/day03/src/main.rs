use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::time::Instant;
use std::{fs, str::Lines};

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn is_symbol(input: char) -> bool {
    !input.is_numeric() && input != '.'
}

fn contains_symbol(line: &str, x_start: i32, x_len: i32) -> bool {
    let x = if x_start > 0 { x_start - 1 } else { 0 };
    let x_end = if x_start > 0 {
        x + x_len + 2
    } else {
        x + x_len + 1
    };
    let x_end = x_end as usize;
    let x_end = if x_end >= line.len() {
        line.len()
    } else {
        x_end
    };
    line[x as usize..x_end].chars().any(is_symbol)
}

fn is_touching_symbol(input: Lines, y: i32, x_start: i32, x_len: i32) -> bool {
    let lines = input
        .enumerate()
        .filter(|(i, _)| i.abs_diff(y as usize) <= 1)
        .map(|(_, x)| x);
    let mut lines_mut = lines.into_iter();
    lines_mut.any(|line| contains_symbol(line, x_start, x_len))
}

fn solve1(filename: &str) -> i32 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut output = String::new();

    let mut sum = 0;

    for (y, line) in lines.clone().enumerate() {
        let mut x_start = -1i32;
        let mut x_len = 0i32;
        let mut value = 0i32;

        for (x, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if x_start == -1 {
                    x_start = x as i32;
                }
                value *= 10;
                value += ch.to_digit(10).unwrap() as i32;
                x_len += 1;
            } else if x_start != -1 {
                if is_touching_symbol(lines.clone(), y as i32, x_start, x_len) {
                    sum += value;
                    output.push_str(value.to_string().as_str());
                    output.push('.');
                    // println!("Counting value [{x_start},{y}]: {value}");
                } else {
                    std::iter::repeat('.')
                        .take((x_len + 1) as usize)
                        .for_each(|c| output.push(c));
                }

                x_start = -1;
                x_len = 0;
                value = 0;
            } else {
                output.push('.');
            }
        }
        if x_start != -1 {
            if is_touching_symbol(lines.clone(), y as i32, x_start, x_len) {
                sum += value;
                output.push_str(value.to_string().as_str());
                // println!("Counting value [{x_start},{y}]: {value}");
            } else {
                std::iter::repeat('.')
                    .take(x_len as usize)
                    .for_each(|c| output.push(c));
            }
        }
        output.push('\n');
    }

    sum
}

fn contains_gear_symbol(line: &str, y: usize, x_start: i32, x_len: i32) -> Option<Coords> {
    let x = if x_start > 0 { x_start - 1 } else { 0 };
    let x_end = if x_start > 0 {
        x + x_len + 2
    } else {
        x + x_len + 1
    };
    let x_end = x_end as usize;
    let x_end = if x_end >= line.len() {
        line.len()
    } else {
        x_end
    };
    let x_pos = line[x as usize..x_end].chars().position(|ch| ch == '*');
    x_pos.map(|x_pos| Coords {
        x: x as usize + x_pos,
        y,
    })
}

fn is_touching_gear_symbol(input: Lines, y: i32, x_start: i32, x_len: i32) -> Option<Coords> {
    let lines = input
        .enumerate()
        .filter(|(i, _)| i.abs_diff(y as usize) <= 1);
    let mut lines_mut = lines.into_iter();
    lines_mut.find_map(|(y_line, line)| contains_gear_symbol(line, y_line, x_start, x_len))
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

fn solve2(filename: &str) -> i32 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut gears: HashMap<Coords, i32> = HashMap::new();
    let mut sum = 0;

    for (y, line) in lines.clone().enumerate() {
        let mut x_start = -1i32;
        let mut x_len = 0i32;
        let mut value = 0i32;

        for (x, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if x_start == -1 {
                    x_start = x as i32;
                }
                value *= 10;
                value += ch.to_digit(10).unwrap() as i32;
                x_len += 1;
            } else if x_start != -1 {
                let coords = is_touching_gear_symbol(lines.clone(), y as i32, x_start, x_len);
                if let Some(coords) = coords {
                    match gears.entry(coords) {
                        Occupied(v) => {
                            sum += value * v.get();
                            v.remove();
                        }
                        Vacant(v) => {
                            v.insert(value);
                        }
                    }
                }

                x_start = -1;
                x_len = 0;
                value = 0;
            }
        }
        if x_start != -1 {
            let coords = is_touching_gear_symbol(lines.clone(), y as i32, x_start, x_len);
            if let Some(coords) = coords {
                match gears.entry(coords) {
                    Occupied(v) => {
                        sum += value * v.get();
                        v.remove();
                    }
                    Vacant(v) => {
                        v.insert(value);
                    }
                }
            }
        }
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

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 4361);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 467835);
    }
}
