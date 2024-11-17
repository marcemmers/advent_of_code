use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Default)]
struct Coord {
    x: i128,
    y: i128,
    z: i128,
}

impl Coord {
    fn new_from_vec(input: &[i128]) -> Self {
        Coord {
            x: input[0],
            y: input[1],
            z: input[2],
        }
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
        let positions: Vec<i128> = position
            .split(',')
            .map(|pos| pos.trim().parse::<i128>().unwrap())
            .collect();
        let velocities: Vec<i128> = velocity
            .split(',')
            .map(|pos| pos.trim().parse::<i128>().unwrap())
            .collect();
        Hailstone {
            position: Coord::new_from_vec(positions.as_slice()),
            velocity: Coord::new_from_vec(velocities.as_slice()),
        }
    }

    fn future_stone(&self, time: i128) -> Self {
        let mut new_pos = self.position;
        new_pos.x += self.velocity.x * time;
        new_pos.y += self.velocity.y * time;
        new_pos.z += self.velocity.z * time;

        Hailstone {
            position: new_pos,
            velocity: self.velocity,
        }
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

        if !(0f64..=1f64).contains(&t) || !(0f64..=1f64).contains(&u) {
            return None;
        }

        let x = x1 as f64 + t * (x2 - x1) as f64;
        let y = y1 as f64 + t * (y2 - y1) as f64;

        // println!("P: {}, {}", x, y);
        Some(Coord {
            x: x as i128,
            y: y as i128,
            z: 0,
        })
    }

    fn to_linear_equation_xy(&self, other: &Self) -> Vec<f64> {
        [
            self.velocity.y - other.velocity.y,
            other.position.y - self.position.y,
            other.velocity.x - self.velocity.x,
            self.position.x - other.position.x,
            other.position.y * other.velocity.x - other.position.x * other.velocity.y
                + self.position.x * self.velocity.y
                - self.position.y * self.velocity.x,
        ]
        .iter()
        .map(|x| *x as f64)
        .collect()
    }

    fn to_linear_equation_xz(&self, other: &Self) -> Vec<f64> {
        [
            self.velocity.z - other.velocity.z,
            other.position.z - self.position.z,
            other.velocity.x - self.velocity.x,
            self.position.x - other.position.x,
            other.position.z * other.velocity.x - other.position.x * other.velocity.z
                + self.position.x * self.velocity.z
                - self.position.z * self.velocity.x,
        ]
        .iter()
        .map(|x| *x as f64)
        .collect()
    }
}

fn solve1(filename: &str, min: i128, max: i128) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::new_from_line).collect();

    // println!("Hailstones: {:?}", hailstones);

    let mut it = hailstones.iter();

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

    sum
}

fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Vec<f64> {
    assert_ne!(matrix.len(), 0);
    assert_ne!(matrix[0].len(), 0);
    let m = matrix.len();
    let n = matrix[0].len();
    assert_eq!(m, n - 1);

    let mut h = 0;
    let mut k = 0;

    // println!("Input: {:?}", matrix);

    while h < m && k < n {
        let i_max = (h..m)
            .map(|i| (i, matrix[i][k].abs()))
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i)
            .unwrap();

        if matrix[i_max][k] == 0f64 {
            k += 1;
        } else {
            let copy = matrix[i_max].clone();
            matrix[i_max] = matrix[h].clone();
            matrix[h] = copy;

            for i in (h + 1)..m {
                let f = matrix[i][k] / matrix[h][k];
                matrix[i][k] = 0f64;
                for j in (k + 1)..n {
                    matrix[i][j] -= matrix[h][j] * f;
                }
            }
            h += 1;
            k += 1;
        }
    }

    for i in (1..m).rev() {
        if matrix[i][i] != 0f64 {
            for j in (0..i).rev() {
                let f = matrix[j][i] / matrix[i][i];
                for k in (0..n).rev() {
                    matrix[j][k] -= f * matrix[i][k];
                }
            }
        }
    }

    // println!("Result: {:?}", matrix);

    // println!("Result: {:?}", result);

    (0..m).map(|i| matrix[i][m] / matrix[i][i]).collect()
}

fn solve2(filename: &str) -> i128 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::new_from_line).collect();

    let mut input = [
        hailstones[0].to_linear_equation_xy(&hailstones[1]),
        hailstones[0].to_linear_equation_xy(&hailstones[2]),
        hailstones[0].to_linear_equation_xy(&hailstones[3]),
        hailstones[0].to_linear_equation_xy(&hailstones[4]),
    ];

    let result_xy = gaussian_elimination(&mut input);

    let mut input2 = [
        hailstones[0].to_linear_equation_xz(&hailstones[1]),
        hailstones[0].to_linear_equation_xz(&hailstones[2]),
        hailstones[0].to_linear_equation_xz(&hailstones[3]),
        hailstones[0].to_linear_equation_xz(&hailstones[4]),
    ];

    let result_xz = gaussian_elimination(&mut input2);

    let x = result_xy[0].round() as i128;
    let y = result_xy[2].round() as i128;
    let z = result_xz[2].round() as i128;

    println!("Result: x={x}, y={y}, z={z}");

    x + y + z
}

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!(
        "Result of 1: {}",
        solve1(PUZZLE_FILENAME, 200000000000000, 400000000000000)
    );
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE_FILENAME));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME, 7, 27), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 47);
    }

    #[test]
    fn test_gauss() {
        let mut matrix = [
            vec![2f64, 1f64, -1f64, 8f64],
            vec![-3f64, -1f64, 2f64, -11f64],
            vec![-2f64, 1f64, 2f64, -3f64],
        ];
        let result = vec![2f64, 3f64, -1f64];
        assert_eq!(
            gaussian_elimination(&mut matrix)
                .iter()
                .map(|x| x.round())
                .collect::<Vec<f64>>(),
            result
        );
    }
}
