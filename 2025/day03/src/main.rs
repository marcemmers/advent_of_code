use std::time::Instant;

fn find_highest_digit(line: &str) -> (usize, u64) {
    for c in (0..=9).rev() {
        if let Some(ch) = line.find(char::from_digit(c, 10).unwrap()) {
            return (ch, c as u64);
        }
    }
    (0, 0)
}

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let (first_pos, first_digit) = find_highest_digit(&line[..line.len() - 1]);
        let (_, second_digit) = find_highest_digit(&line[first_pos + 1..]);

        sum += first_digit * 10 + second_digit;
    }

    sum
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let mut pos = 0;
        let mut joltage = 0;
        for digits_needed in (0..12).rev() {
            let (first_pos, first_digit) =
                find_highest_digit(&line[pos..line.len() - digits_needed]);

            pos += first_pos + 1;
            joltage = (joltage * 10) + first_digit;
        }

        sum += joltage;
    }

    sum
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
        assert_eq!(solve1(EXAMPLE), 357);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 3121910778619);
    }
}
