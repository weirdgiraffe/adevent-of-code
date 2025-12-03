use anyhow::{Context, Result, anyhow};
use std::env;
use std::fs::File;
use std::io::Read;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn solve_file<F>(path: &str, f: F) -> Result<i64>
where
    F: Fn(&str) -> Result<i64>,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    solve_str(&content, f)
}

fn solve_str<F>(content: &str, joltage: F) -> Result<i64>
where
    F: Fn(&str) -> Result<i64>,
{
    content.lines().map(|line| joltage(line)).sum()
}

fn part1(bank: &str) -> Result<i64> {
    joltage(bank, 2)
}

fn part2(bank: &str) -> Result<i64> {
    joltage(bank, 12)
}

fn joltage(bank: &str, count: usize) -> Result<i64> {
    let l: Vec<u32> = bank
        .chars()
        .map(|ch| match ch.to_digit(10) {
            Some(n) => Ok(n),
            None => Err(anyhow!("not a number {}", ch)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mem = Rc::new(RefCell::new(HashMap::<String, i64>::new()));
    let max = max_of(mem, l.as_slice(), count).unwrap();
    println!("s={} joltage={}", bank, max);
    Ok(max)
}

fn max_of(mem: Rc<RefCell<HashMap<String, i64>>>, l: &[u32], count: usize) -> Option<i64> {
    if count == 0 || l.len() < count {
        return None;
    }

    if count == 1 {
        return l.iter().max().and_then(|x| Some(*x as i64));
    }

    let key = l.iter().map(|n| n.to_string()).collect::<String>() + "" + &count.to_string();
    if let Some(&stored) = mem.borrow().get(&key) {
        return Some(stored);
    }

    let mut max: i64 = 0;
    for i in 0..l.len() - 1 {
        let next = max_of(mem.clone(), &l[i + 1..], count - 1);
        if next.is_none() {
            break;
        }
        let this: i64 = l[i].into();
        let curr: i64 = this * 10_i64.pow((count - 1).try_into().unwrap());
        if curr + next.unwrap() > max {
            max = curr + next.unwrap();
        }
    }

    mem.borrow_mut().insert(key, max);

    // println!("l={:?} count={} max={}", l, count, max);
    Some(max)
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
        987654321111111
        811111111111119
        234234234234278
        818181911112111
        "
    };

    #[test]
    fn test_part1() {
        let actual = solve_str(INPUT, part1).expect("must solve");
        assert_eq!(357, actual);
    }

    #[test]
    fn test_part2() {
        let actual = solve_str(INPUT, part2).expect("must solve");
        assert_eq!(3121910778619, actual);
    }
}
