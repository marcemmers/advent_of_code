use std::fs;
use std::time::Instant;

struct Grid {
    grid: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

impl Grid {
    fn new() -> Self {
        Grid { grid: Vec::new() }
    }

    fn get_value(&self, point: Point) -> Option<char> {
        self.grid
            .get(point.y as usize)?
            .get(point.x as usize)
            .copied()
    }

    fn insert(&mut self, new: Vec<char>) {
        self.grid.insert(0, new);
    }
}

impl Point {
    fn step(&self, heading: Heading) -> Point {
        match heading {
            Heading::North => Point {
                x: self.x,
                y: self.y + 1,
            },
            Heading::South => Point {
                x: self.x,
                y: self.y - 1,
            },
            Heading::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Heading::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct EnergizedPoint {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl EnergizedPoint {
    fn new() -> Self {
        EnergizedPoint {
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }

    fn energize(&mut self, heading: Heading) {
        match heading {
            Heading::North => self.north = true,
            Heading::South => self.south = true,
            Heading::East => self.east = true,
            Heading::West => self.west = true,
        }
    }

    fn has_heading(&self, heading: Heading) -> bool {
        match heading {
            Heading::North => self.north,
            Heading::South => self.south,
            Heading::East => self.east,
            Heading::West => self.west,
        }
    }

    fn is_energized(&self) -> bool {
        self.north || self.south || self.east || self.west
    }
}

struct EnergizedGrid {
    grid: Vec<Vec<EnergizedPoint>>,
}

impl EnergizedGrid {
    fn new_from_size(x: usize, y: usize) -> Self {
        EnergizedGrid {
            grid: vec![vec![EnergizedPoint::new(); x]; y],
        }
    }

    fn energize(&mut self, point: Point, heading: Heading) -> Option<bool> {
        self.grid
            .get_mut(point.y as usize)?
            .get_mut(point.x as usize)?
            .energize(heading);
        Some(true)
    }

    fn has_heading(&self, point: Point, heading: Heading) -> Option<bool> {
        Some(
            self.grid
                .get(point.y as usize)?
                .get(point.x as usize)?
                .has_heading(heading),
        )
    }

    fn count_energized(&self) -> usize {
        self.grid
            .iter()
            .map(|line| {
                line.iter().fold(0, |acc, item| {
                    if item.is_energized() {
                        acc + 1usize
                    } else {
                        acc
                    }
                })
            })
            .sum()
    }
}

fn make_grid(filename: &str) -> Grid {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut grid: Grid = Grid::new();

    for line in lines {
        grid.insert(line.chars().collect());
    }
    return grid;
}

fn trace_light(grid: &Grid, start: Point, heading: Heading, energized_grid: &mut EnergizedGrid) {
    let mut cur_pos = start;
    let mut heading = heading;

    loop {
        cur_pos = cur_pos.step(heading);
        if let Some(has_heading) = energized_grid.has_heading(cur_pos, heading) {
            if has_heading {
                break;
            }
        } else {
            break;
        }
        if energized_grid.energize(cur_pos, heading) == None {
            break;
        }
        let next_val = grid.get_value(cur_pos).expect("Should be valid pos");
        // println!("Cur pos: {:?}, heading: {:?}, found: {next_val}", cur_pos, heading);
        heading = match heading {
            Heading::North => match next_val {
                '-' => {
                    trace_light(grid, cur_pos, Heading::East, energized_grid);
                    Heading::West
                }
                '\\' => Heading::West,
                '/' => Heading::East,
                _ => Heading::North,
            },
            Heading::South => match next_val {
                '-' => {
                    trace_light(grid, cur_pos, Heading::East, energized_grid);
                    Heading::West
                }
                '/' => Heading::West,
                '\\' => Heading::East,
                _ => Heading::South,
            },
            Heading::East => match next_val {
                '|' => {
                    trace_light(grid, cur_pos, Heading::North, energized_grid);
                    Heading::South
                }
                '/' => Heading::North,
                '\\' => Heading::South,
                _ => Heading::East,
            },
            Heading::West => match next_val {
                '|' => {
                    trace_light(grid, cur_pos, Heading::North, energized_grid);
                    Heading::South
                }
                '\\' => Heading::North,
                '/' => Heading::South,
                _ => Heading::West,
            },
        };
    }
}

fn solve1(filename: &str) -> u64 {
    let grid = make_grid(filename);

    let start = Point {
        x: -1,
        y: grid.grid.len() as i32 - 1,
    };

    let mut energized = EnergizedGrid::new_from_size(grid.grid[0].len(), grid.grid.len());
    trace_light(&grid, start, Heading::East, &mut energized);

    return energized.count_energized() as u64;
}

fn solve2(filename: &str) -> u64 {
    let grid = make_grid(filename);

    let start_west = (0..grid.grid.len()).map(|y| (Point { x: -1, y: y as i32 }, Heading::East));
    let start_east = (0..grid.grid.len()).map(|y| {
        (
            Point {
                x: grid.grid[0].len() as i32,
                y: y as i32,
            },
            Heading::West,
        )
    });
    let start_north = (0..grid.grid[0].len()).map(|x| {
        (
            Point {
                x: x as i32,
                y: grid.grid.len() as i32,
            },
            Heading::South,
        )
    });
    let start_south =
        (0..grid.grid[0].len()).map(|x| (Point { x: x as i32, y: -1 }, Heading::North));

    return start_west
        .chain(start_east)
        .chain(start_north)
        .chain(start_south)
        .map(|(start, heading)| {
            let mut energized = EnergizedGrid::new_from_size(grid.grid[0].len(), grid.grid.len());
            trace_light(&grid, start, heading, &mut energized);
            return energized.count_energized() as u64;
        })
        .max()
        .unwrap();
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
        assert_eq!(solve1(EXAMPLE_FILENAME), 46);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 51);
    }
}
