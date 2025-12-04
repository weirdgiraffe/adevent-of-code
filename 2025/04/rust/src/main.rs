use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Read;

fn solve_file<F>(path: &str, f: F) -> Result<i64>
where
    F: Fn(&mut Grid) -> i64,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(solve_str(&content, f))
}

fn solve_str<F>(content: &str, could_be_accessed: F) -> i64
where
    F: Fn(&mut Grid) -> i64,
{
    let mut grid = Grid::from_str(content);
    println!("{:?}", grid);
    could_be_accessed(&mut grid)
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl Grid {
    // Relative neighbor offsets (8 directions)
    const NEIGHBORS: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn from_str(s: &str) -> Self {
        let mut cols: usize = 0;
        let mut rows: usize = 0;
        let data: Vec<u8> = s
            .lines()
            .map(|line| {
                if cols == 0 {
                    cols = line.len();
                }
                rows += 1;
                line.chars()
            })
            .flatten()
            .map(|ch| match ch {
                '@' => 1,
                _ => 0,
            })
            .collect::<Vec<_>>();
        let mut grid = Self { rows, cols, data };
        for pos in grid.rolls().collect::<Vec<_>>() {
            let i = grid.index(&pos).unwrap();
            grid.data[i] += grid.neighbor_rolls(&pos).count() as u8;
        }

        grid
    }

    fn neighbor_rolls<'a>(&'a self, pos: &'a Position) -> impl Iterator<Item = Position> + use<'a> {
        Self::NEIGHBORS.iter().filter_map(|(dy, dx)| {
            // Use checked_add_signed to stay in usize without casts back and forth
            let ny = match pos.y.checked_add_signed(*dy) {
                Some(v) if v < self.rows => v,
                _ => return None,
            };
            let nx = match pos.x.checked_add_signed(*dx) {
                Some(v) if v < self.cols => v,
                _ => return None,
            };
            let np = Position { x: nx, y: ny };
            if self.data[self.index(&np).unwrap()] > 0 {
                return Some(np);
            }
            None
        })
    }

    fn rolls<'a>(&'a self) -> impl Iterator<Item = Position> + use<'a> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, x)| **x > 0)
            .map(|(i, _)| Position {
                y: i / self.cols,
                x: i % self.cols,
            })
    }

    fn index(&self, pos: &Position) -> Option<usize> {
        if pos.x > self.cols {
            return None;
        }
        if pos.y > self.rows {
            return None;
        }
        Some(pos.y * self.cols + pos.x)
    }

    fn get_neighbors(&self, p: &Position) -> Option<u8> {
        Some(self.data[self.index(p)?].max(1) - 1)
    }

    fn del(&mut self, pos: &Position) {
        if let Some(i) = self.index(pos) {
            if let Some(elem) = self.data.get_mut(i) {
                *elem = 0;
            }
            self.neighbor_rolls(pos)
                .collect::<Vec<_>>()
                .iter()
                .all(|np| {
                    let ni = self.index(np).unwrap();
                    if let Some(elem) = self.data.get_mut(ni) {
                        *elem -= 1
                    }
                    true
                });
        }
    }
}

fn part1(grid: &mut Grid) -> i64 {
    grid.rolls()
        .filter(|pos| grid.get_neighbors(pos).is_some_and(|x| x < 4))
        .inspect(|x| {
            println!(
                "found: pos={:?} rolls={}",
                x,
                grid.get_neighbors(x).unwrap()
            )
        })
        .count() as i64
}

fn part2(grid: &mut Grid) -> i64 {
    let mut count: i64 = 0;
    loop {
        let to_remove: Vec<Position> = grid
            .rolls()
            .filter(|pos| grid.get_neighbors(pos).is_some_and(|x| x < 4))
            .inspect(|x| {
                println!(
                    "found: pos={:?} rolls={}",
                    x,
                    grid.get_neighbors(x).unwrap()
                )
            })
            .collect();

        if to_remove.len() == 0 {
            return count;
        }

        count += to_remove.iter().fold(0i64, |acc, pos| {
            grid.del(pos);
            acc + 1
        });
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).context("missing file path argument")?;
    match args.get(2).map(|s| s.as_str()).context("missing part")? {
        "part1" => {
            println!("solution: {:#?}", solve_file(path, part1)?)
        }
        "part2" => {
            println!("solution: {:#?}", solve_file(path, part2)?)
        }
        s => anyhow::bail!("unexpected part: {}", s),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;
    const INPUT: &str = indoc! {
        "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        "
    };

    #[test]
    fn test_part1() {
        let actual = solve_str(INPUT, part1);
        assert_eq!(13, actual);
    }
    #[test]
    fn test_part2() {
        let actual = solve_str(INPUT, part2);
        assert_eq!(43, actual);
    }
}
