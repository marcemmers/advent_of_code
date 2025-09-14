use std::collections::{HashSet, VecDeque};

use crate::{Direction, Grid, Position};

pub fn bfs_search(
    grid: &Grid,
    start: Position,
    goal: Position,
    is_allowed: impl Fn(&Grid, Position, Position) -> bool,
) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut open = VecDeque::new();

    open.push_back((start, 0));

    while let Some((pos, cost)) = open.pop_front() {
        if pos == goal {
            return Some(cost);
        }

        if !visited.insert(pos) {
            continue;
        }

        for dir in Direction::all_directions() {
            let neighbor = pos + dir;
            if visited.contains(&neighbor) || !is_allowed(grid, pos, neighbor) {
                continue;
            }

            open.push_back((neighbor, cost + 1));
        }
    }

    None
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

        let path = bfs_search(
            &grid,
            grid.find_one('S').unwrap(),
            grid.find_one('E').unwrap(),
            |grid, _, pos| matches!(grid.get(pos), Some('.') | Some('E')),
        );

        assert_eq!(path.unwrap(), 7);
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

        let path = bfs_search(
            &grid,
            grid.find_one('S').unwrap(),
            grid.find_one('E').unwrap(),
            |grid, _, pos| matches!(grid.get(pos), Some('.') | Some('E')),
        );

        assert_eq!(path.unwrap(), 9);
    }

    #[test]
    fn straight_line() {
        let grid = r"
S..x..x......
.............
...x.....x..E
";
        let grid = Grid::from_text(grid.trim());

        let path = bfs_search(
            &grid,
            grid.find_one('S').unwrap(),
            grid.find_one('E').unwrap(),
            |grid, _, pos| matches!(grid.get(pos), Some('.') | Some('E')),
        );

        assert_eq!(path.unwrap(), 14);
    }
}
