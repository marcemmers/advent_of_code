#![feature(ascii_char)]

use std::{ascii, time::Instant};

fn get_column_indices(input: &str) -> Vec<usize> {
    let lines: Vec<&str> = input.lines().collect();

    let mut output = Vec::new();

    for idx in 0..lines[0].len() {
        if lines.iter().all(|line| line.as_bytes()[idx] == b' ') {
            output.push(idx);
        }
    }

    output
}

fn get_columns(input: &str) -> Vec<Vec<&str>> {
    let indices = get_column_indices(input);

    let lines: Vec<&str> = input.lines().collect();

    let mut columns = Vec::new();
    columns.resize(indices.len() + 1, Vec::new());

    for mut line in lines {
        for (idx, pos) in indices.iter().rev().enumerate() {
            let (remainder, part) = line.split_at(*pos);
            line = remainder;
            columns[idx].push(part);
        }
        let (part, _) = line.split_at(*indices.first().unwrap());
        columns.last_mut().unwrap().push(part);
    }

    columns
}

fn solve1(input: &str) -> u64 {
    let mut total = 0;
    let columns = get_columns(input);

    for column in columns {
        let (op, numbers) = column.as_slice().split_last().unwrap();

        let numbers = numbers.iter().map(|nr| nr.trim().parse::<u64>().unwrap());

        total += match op.trim() {
            "+" => numbers.sum(),
            "*" => numbers.product(),
            _ => 0,
        };
    }

    total
}

fn solve2(input: &str) -> u64 {
    let lines: Vec<&[ascii::Char]> = input.lines().map(|line| line.as_ascii().unwrap()).collect();

    let mut input = Vec::with_capacity(input.len());

    for idx in 0..lines[0].len() {
        for line in lines.iter().rev() {
            input.push(line[idx]);
        }
    }

    let mut total = 0;
    let mut subtotal = 0;
    let mut current_value = 0;
    let mut current_decimal = 1;
    let mut op = ' ';

    for ch in input {
        match ch.to_char() {
            '*' | '+' => {
                total += subtotal;
                subtotal = 0;
                op = ch.to_char();
            }
            ' ' if current_value != 0 => {
                if op == '+' || subtotal == 0 {
                    subtotal += current_value;
                } else if op == '*' {
                    subtotal *= current_value;
                }
                current_value = 0;
                current_decimal = 1;
            }
            x if x.is_ascii_digit() => {
                current_value += x.to_digit(10).unwrap() as u64 * current_decimal;
                current_decimal *= 10;
            }
            _ => (),
        }
    }

    // Add last one as it does not have an operator
    total += subtotal;

    total
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 4277556);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 3263827);
    }
}
