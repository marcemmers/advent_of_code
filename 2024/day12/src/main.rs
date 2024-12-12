use std::{collections::HashSet, time::Instant};

use grid::{Direction, Grid, Position};

fn calculate_plot_cost(grid: &mut Grid, start: Position) -> u64 {
    let crop = grid.get(start).unwrap();

    let mut to_visit: HashSet<Position> = HashSet::new();
    let mut plot: HashSet<Position> = HashSet::new();

    to_visit.insert(start);
    plot.insert(start);

    let mut perimiter = 0;

    while let Some(pos) = to_visit.iter().next().cloned() {
        to_visit.remove(&pos);
        for dir in Direction::all_directions() {
            let step = pos.step(dir);
            if grid.get(step) == Some(crop) {
                if plot.insert(step) {
                    to_visit.insert(step);
                }
            } else {
                perimiter += 1;
            }
        }
    }

    plot.iter()
        .for_each(|pos| *grid.get_mut(*pos).unwrap() = '.');

    plot.len() as u64 * perimiter
}

fn solve1(input: &str) -> u64 {
    let mut grid = Grid::from_text(input);

    let mut sum = 0;
    loop {
        let item = grid.iter().find(|(_, ch)| *ch != '.');
        if item.is_none() {
            break;
        }

        let (pos, _) = item.unwrap();

        sum += calculate_plot_cost(&mut grid, pos);
    }

    sum
}

fn trace_perimiter(plot: &HashSet<Position>) -> u64 {
    if plot.len() == 1 {
        return 4;
    }

    let mut pos = plot.iter().next().cloned().unwrap();

    // Find some up left corner
    loop {
        if let Some(step) = plot.get(&pos.step(Direction::Left)) {
            pos = *step;
        } else if let Some(step) = plot.get(&pos.step(Direction::Up)) {
            pos = *step;
        } else {
            break;
        }
    }

    let start = pos;

    println!("Start at {:?}", start);

    let mut sides = 1;

    // Start going right
    let mut dir = Direction::Right;

    // If there is nothing right, go down
    if plot.get(&pos.step(dir)).is_none() {
        dir = Direction::Down;
        sides += 1;
    }

    pos = pos.step(dir);
    // for _ in 0..50 {
    loop {
        if let Some(next) = plot.get(&pos.step(dir.turn_left())) {
            pos = *next;
            dir = dir.turn_left();
            sides += 1;
        } else if pos == start {
            break;
        } else if let Some(next) = plot.get(&pos.step(dir)) {
            pos = *next;
        } else {
            dir = dir.turn_right();
            sides += 1;
        }
        // println!("Pos {:?}", pos);
    }

    if dir == Direction::Left {
        sides += 1;
    }

    sides
}

fn calculate_plot_cost_with_bulk(grid: &mut Grid, start: Position) -> u64 {
    let crop = grid.get(start).unwrap();

    let mut to_visit: HashSet<Position> = HashSet::new();
    let mut plot: HashSet<Position> = HashSet::new();

    to_visit.insert(start);
    plot.insert(start);

    while let Some(pos) = to_visit.iter().next().cloned() {
        to_visit.remove(&pos);
        for dir in Direction::all_directions() {
            let step = pos.step(dir);
            if grid.get(step) == Some(crop) && plot.insert(step) {
                to_visit.insert(step);
            }
        }
        // println!("To visit for {}: {:?}", crop, to_visit);
    }

    let perimiter = trace_perimiter(&plot);

    println!(
        "Found plot '{}' with cost {} * {} = {}",
        crop,
        plot.len(),
        perimiter,
        plot.len() as u64 * perimiter
    );

    // if perimiter % 2 != 0 {
    plot.iter()
        .for_each(|pos| *grid.get_mut(*pos).unwrap() = '+');

    grid.print();
    // }

    plot.iter()
        .for_each(|pos| *grid.get_mut(*pos).unwrap() = '.');

    plot.len() as u64 * perimiter
}

fn solve2(input: &str) -> u64 {
    let mut grid = Grid::from_text(input);

    let mut sum = 0;
    loop {
        let item = grid.iter().find(|(_, ch)| *ch != '.');
        if item.is_none() {
            break;
        }

        let (pos, _) = item.unwrap();

        sum += calculate_plot_cost_with_bulk(&mut grid, pos);
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

    const CUSTOM_EXAMPLE: &str = include_str!("./custom1.txt");

    #[test]
    fn test_once() {
        assert_eq!(
            calculate_plot_cost_with_bulk(&mut Grid::from_text(EXAMPLE), Position { x: 0, y: 0 }),
            10
        );
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 1930);
    }

    #[test]
    fn test_custom1() {
        assert_eq!(solve2(CUSTOM_EXAMPLE), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 1206);
    }
}
