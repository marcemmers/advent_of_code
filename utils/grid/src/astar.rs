use std::collections::{BinaryHeap, HashMap};

use crate::{Direction, Grid, Position};

#[derive(Eq, PartialEq, Debug)]
struct OpenPosition {
    pos: Position,
    g_score: i32,
    f_score: i32,
}

impl PartialOrd for OpenPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

pub fn calculate_path(
    grid: &Grid,
    start: Position,
    goal: Position,
    is_allowed: impl Fn(&Grid, Position, Position) -> bool,
) -> Option<Vec<Position>> {
    let grid_positions = grid.height() * grid.width();

    let mut open_list = BinaryHeap::with_capacity(grid_positions);
    let mut came_from = HashMap::with_capacity(grid_positions);
    let mut closed_list = HashMap::with_capacity(grid_positions);

    open_list.push(OpenPosition {
        pos: start,
        g_score: 0,
        f_score: h(goal, start),
    });

    while let Some(current) = open_list.pop() {
        if current.pos == goal {
            return Some(reconstruct_path(&came_from, current.pos));
        }

        closed_list.insert(current.pos, current.g_score);

        for dir in Direction::all_directions() {
            let neighbor = current.pos + dir;
            if !is_allowed(grid, current.pos, neighbor)
                || open_list.iter().any(|x| x.pos == neighbor)
            {
                continue;
            }

            let tentative = current.g_score + 1;
            if let Some(g_score) = closed_list.get(&neighbor)
                && tentative >= *g_score
            {
                continue;
            }

            came_from.insert(neighbor, current.pos);
            open_list.push(OpenPosition {
                pos: neighbor,
                g_score: tentative,
                f_score: tentative + h(goal, neighbor),
            });
        }
    }
    None
}

fn h(goal: Position, pos: Position) -> i32 {
    let distance = goal.distance_xy(pos);
    distance.x.abs() + distance.y.abs()
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = Vec::new();
    path.push(current);
    while let Some(new) = came_from.get(&current) {
        current = *new;
        path.push(current);
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let grid = r"
S....
x..x.
xxxx.
....E
";
        let grid = Grid::from_text(grid.trim());

        let path = calculate_path(
            &grid,
            grid.find_one('S').unwrap(),
            grid.find_one('E').unwrap(),
            |grid, _, pos| matches!(grid.get(pos), Some('.') | Some('E')),
        );

        assert_eq!(path.unwrap().len(), 8);
    }

    #[test]
    fn going_up() {
        let grid = r"
.....
.xxx.
Sx.x.
...xE
";
        let grid = Grid::from_text(grid.trim());

        let path = calculate_path(
            &grid,
            grid.find_one('S').unwrap(),
            grid.find_one('E').unwrap(),
            |grid, _, pos| matches!(grid.get(pos), Some('.') | Some('E')),
        );

        assert_eq!(path.unwrap().len(), 10);
    }

    #[test]
    fn straight_line() {
        let grid = r"
S..x..x......
.............
...x.....x..E
";
        let grid = Grid::from_text(grid.trim());

        let path = calculate_path(
            &grid,
            grid.find_one('S').unwrap(),
            grid.find_one('E').unwrap(),
            |grid, _, pos| matches!(grid.get(pos), Some('.') | Some('E')),
        );

        assert_eq!(path.unwrap().len(), 15);
    }
}
