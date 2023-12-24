use std::time::Instant;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i128,
    y: i128,
    z: i128
}

impl Coord {
    fn new() -> Self {
        Coord{x:0, y: 0, z: 0}
    }

    fn new_from_vec(input: &[i128]) -> Self {
        Coord{x: input[0], y: input[1], z: input[2]}
    }
}


#[derive(Debug)]
struct Hailstone {
    position: Coord,
    velocity: Coord,
}

impl Hailstone {
    fn new_from_line(line: &str) -> Self {
        let (position, velocity) = line.split_once('@').unwrap();
        let positions: Vec<i128> = position.split(',').map(|pos| pos.trim().parse::<i128>().unwrap()).collect();
        let velocities: Vec<i128> = velocity.split(',').map(|pos| pos.trim().parse::<i128>().unwrap()).collect();
        Hailstone{position: Coord::new_from_vec(positions.as_slice()), velocity: Coord::new_from_vec(velocities.as_slice())}
    }

    fn future_stone(&self, time: i128) -> Self {
        let mut new_pos = self.position;
        new_pos.x += self.velocity.x * time;
        new_pos.y += self.velocity.y * time;
        new_pos.z += self.velocity.z * time;

        Hailstone{position: new_pos, velocity: self.velocity}
    }

    fn intersect_xy(&self, other: &Hailstone) -> Option<Coord> {
        let x1 = self.position.x;
        let y1 = self.position.y;
        let x3 = other.position.x;
        let y3 = other.position.y;

        let future_self = self.future_stone(1_000_000_000_000_000);
        let future_other = other.future_stone(1_000_000_000_000_000);
        let x2 = future_self.position.x;
        let y2 = future_self.position.y;
        let x4 = future_other.position.x;
        let y4 = future_other.position.y;

        let den = ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)) as f64;

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64 / den;
        let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) as f64 / den;

        // println!("t: {t}");
        // println!("u: {u}");

        if t < 0f64 || t > 1f64 || u < 0f64 || u > 1f64 {
            return None;
        }

        let x = x1 as f64 + t * (x2 - x1) as f64;
        let y = y1 as f64 + t * (y2 - y1) as f64;

        // println!("P: {}, {}", x, y);
        return Some(Coord{x: x as i128, y: y as i128, z: 0});
    }

}


fn solve1(filename: &str, min: i128, max: i128) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let hailstones: Vec<Hailstone> = input.lines().map(|line| Hailstone::new_from_line(line)).collect();

    println!("Hailstones: {:?}", hailstones);

    let mut it = hailstones.iter();

     // Start one from the start so we don't combine with ourselves.

    let mut sum = 0;

    for stone in hailstones.iter() {
        it.next();
        for stone2 in it.clone() {
            if let Some(coord) = stone.intersect_xy(stone2) {
                if coord.x >= min && coord.x <= max && coord.y >= min && coord.y <= max {
                    sum += 1;
                }
            }
        }
    }

    return sum;
}


fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    return 0;
}


const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE_FILENAME, 200000000000000, 400000000000000));
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
        assert_eq!(solve1(EXAMPLE_FILENAME, 7, 27), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}