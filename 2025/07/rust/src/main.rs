use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Read;

fn solve_file<F>(path: &str, f: F) -> Result<u64>
where
    F: Fn(&str) -> u64,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(solve_str(&content, f))
}

fn solve_str<F>(content: &str, f: F) -> u64
where
    F: Fn(&str) -> u64,
{
    f(content)
}

type Row = Vec<char>;
type Grid = Vec<Row>;

fn parse(input: &str) -> Grid {
    input.lines().fold(Grid::new(), |mut grid, line| {
        let mut row: Row = Vec::new();
        line.chars().for_each(|ch| row.push(ch));
        grid.push(row);
        grid
    })
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn part1(input: &str) -> u64 {
    let grid = parse(input);
    let (j, _) = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == 'S')
        .unwrap();

    let mem = Rc::new(RefCell::new(HashMap::<(usize, usize), bool>::new()));
    count1(mem, &grid, 0, j)
}

fn count1(
    visited: Rc<RefCell<HashMap<(usize, usize), bool>>>,
    grid: &Grid,
    i: usize,
    j: usize,
) -> u64 {
    if i == grid.len() {
        return 0;
    }
    if j == grid[0].len() || j == 0 {
        return 0;
    }

    if let Some(_) = visited.borrow().get(&(i, j)) {
        return 0;
    }

    let res = match grid[i][j] {
        '^' => {
            1 + count1(visited.clone(), &grid, i + 1, j.saturating_sub(1))
                + count1(visited.clone(), &grid, i + 1, j.saturating_add(1))
        }
        _ => count1(visited.clone(), &grid, i + 1, j),
    };

    visited.borrow_mut().insert((i, j), true);
    res
}

fn part2(input: &str) -> u64 {
    let grid = parse(input);
    let (j, _) = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == 'S')
        .unwrap();

    let mem = Rc::new(RefCell::new(HashMap::<(usize, usize), u64>::new()));
    count2(mem, &grid, 0, j)
}

fn count2(
    visited: Rc<RefCell<HashMap<(usize, usize), u64>>>,
    grid: &Grid,
    i: usize,
    j: usize,
) -> u64 {
    if i == grid.len() {
        return 1;
    }
    if j == grid[0].len() || j == 0 {
        return 1;
    }

    if let Some(n) = visited.borrow().get(&(i, j)) {
        return *n;
    }

    let res = match grid[i][j] {
        '^' => {
            count2(visited.clone(), &grid, i + 1, j.saturating_sub(1))
                + count2(visited.clone(), &grid, i + 1, j.saturating_add(1))
        }
        _ => count2(visited.clone(), &grid, i + 1, j),
    };

    visited.borrow_mut().insert((i, j), res);
    res
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
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
        "
    };

    #[test]
    fn test_part1() {
        let actual = solve_str(INPUT, part1);
        assert_eq!(21, actual);
    }
    #[test]
    fn test_part2() {
        let actual = solve_str(INPUT, part2);
        assert_eq!(40, actual);
    }
}

// wrong: 8907711267206
//        8907730960817
