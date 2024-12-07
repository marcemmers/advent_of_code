use grid::{Direction, Grid, Position};
use std::fs;
use std::time::Instant;

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

    fn energize(&mut self, heading: Direction) {
        match heading {
            Direction::Up => self.north = true,
            Direction::Down => self.south = true,
            Direction::Right => self.east = true,
            Direction::Left => self.west = true,
        }
    }

    fn has_heading(&self, heading: Direction) -> bool {
        match heading {
            Direction::Up => self.north,
            Direction::Down => self.south,
            Direction::Right => self.east,
            Direction::Left => self.west,
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

    fn energize(&mut self, point: Position, heading: Direction) -> Option<bool> {
        self.grid
            .get_mut(point.y as usize)?
            .get_mut(point.x as usize)?
            .energize(heading);
        Some(true)
    }

    fn has_heading(&self, point: Position, heading: Direction) -> Option<bool> {
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

    Grid::from_text(&input)
}

fn trace_light(
    grid: &Grid,
    start: Position,
    heading: Direction,
    energized_grid: &mut EnergizedGrid,
) {
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
        if energized_grid.energize(cur_pos, heading).is_none() {
            break;
        }
        let next_val = grid.get(cur_pos).expect("Should be valid pos");
        // println!(
        //     "Cur pos: {:?}, heading: {:?}, found: {next_val}",
        //     cur_pos, heading
        // );
        heading = match heading {
            Direction::Up => match next_val {
                '-' => {
                    trace_light(grid, cur_pos, Direction::Right, energized_grid);
                    Direction::Left
                }
                '\\' => Direction::Left,
                '/' => Direction::Right,
                _ => Direction::Up,
            },
            Direction::Down => match next_val {
                '-' => {
                    trace_light(grid, cur_pos, Direction::Right, energized_grid);
                    Direction::Left
                }
                '/' => Direction::Left,
                '\\' => Direction::Right,
                _ => Direction::Down,
            },
            Direction::Right => match next_val {
                '|' => {
                    trace_light(grid, cur_pos, Direction::Up, energized_grid);
                    Direction::Down
                }
                '/' => Direction::Up,
                '\\' => Direction::Down,
                _ => Direction::Right,
            },
            Direction::Left => match next_val {
                '|' => {
                    trace_light(grid, cur_pos, Direction::Up, energized_grid);
                    Direction::Down
                }
                '\\' => Direction::Up,
                '/' => Direction::Down,
                _ => Direction::Left,
            },
        };
    }
}

fn solve1(filename: &str) -> u64 {
    let grid = make_grid(filename);

    let start = Position { x: -1, y: 0 };

    let mut energized = EnergizedGrid::new_from_size(grid.width(), grid.height());
    trace_light(&grid, start, Direction::Right, &mut energized);

    energized.count_energized() as u64
}

fn solve2(filename: &str) -> u64 {
    let grid = make_grid(filename);

    let start_west =
        (0..grid.height()).map(|y| (Position { x: -1, y: y as i32 }, Direction::Right));
    let start_east = (0..grid.height()).map(|y| {
        (
            Position {
                x: grid.width() as i32,
                y: y as i32,
            },
            Direction::Left,
        )
    });
    let start_north = (0..grid.width()).map(|x| (Position { x: x as i32, y: -1 }, Direction::Down));
    let start_south = (0..grid.width()).map(|x| {
        (
            Position {
                x: x as i32,
                y: grid.height() as i32,
            },
            Direction::Up,
        )
    });

    start_west
        .chain(start_east)
        .chain(start_north)
        .chain(start_south)
        .map(|(start, heading)| {
            let mut energized = EnergizedGrid::new_from_size(grid.width(), grid.height());
            trace_light(&grid, start, heading, &mut energized);
            energized.count_energized() as u64
        })
        .max()
        .unwrap()
}

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

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

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 46);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 51);
    }
}
