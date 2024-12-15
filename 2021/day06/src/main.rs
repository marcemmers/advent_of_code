use std::time::Instant;

fn solve(input: &str, days: u64) -> u64 {
    let input: Vec<u64> = input.split(',').map(|val| val.parse().unwrap()).collect();

    let mut fishes = [0; 9];
    input.iter().for_each(|val| {
        fishes[*val as usize] += 1;
    });

    for _ in 0..days {
        let mut new_fishes = [0; 9];
        fishes.iter().enumerate().for_each(|(day, fishes)| {
            if day == 0 {
                new_fishes[6] = *fishes;
                new_fishes[8] = *fishes;
            } else {
                new_fishes[day - 1] += fishes;
            }
        });
        fishes = new_fishes;
    }

    fishes.iter().sum()
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve(PUZZLE, 80));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve(PUZZLE, 256));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(EXAMPLE, 80), 5934);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(EXAMPLE, 256), 26984457539);
    }
}
