use anyhow::{Context, Result, anyhow};
use std::env;
use std::fs::File;
use std::io::Read;

fn solve_file<F>(path: &str, f: F) -> Result<i64>
where
    F: Fn(i64) -> bool + Clone,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    solve_str(&content, f)
}

fn part1(id: i64) -> bool {
    let s = id.to_string();
    if s.len() % 2 != 0 {
        return true;
    }
    let (first, second) = s.split_at(s.len() / 2);
    first != second
}

fn part2(id: i64) -> bool {
    let b = id.to_string().into_bytes();
    for i in 1..b.len() {
        if b[i..].starts_with(&b[0..i]) {
            let mut found = true;
            for j in (i..b.len()).step_by(i) {
                if !b[j..].starts_with(&b[0..i]) {
                    found = false
                }
            }
            if found {
                return false;
            }
        }
    }
    return true;
}

fn sum_invalid_ids<F>(range: &str, is_valid_id: F) -> Result<i64>
where
    F: Fn(i64) -> bool,
{
    let parts: Vec<i64> = range
        .split("-")
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| anyhow!("must have valid range definition, got {:?}", range))?;

    let [start, end]: [i64; 2] = parts
        .try_into()
        .map_err(|input| anyhow!("must have 2 numbers, got {:?}", input))?;

    let mut sum: i64 = 0;
    for i in start..end + 1 {
        if !is_valid_id(i) {
            // println!("range [{},{}]: {} is invalid id", start, end, i);
            sum += i
        }
    }
    Ok(sum)
}

fn solve_str<F>(content: &str, is_valid_id: F) -> Result<i64>
where
    F: Fn(i64) -> bool + Clone,
{
    content.lines().try_fold(0, |acc, line| {
        let sum: i64 = line
            .split(",")
            .map(|s| sum_invalid_ids(s, is_valid_id.clone()))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        Ok(acc + sum)
    })
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
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_1() {
        let actual = solve_str(INPUT, part1).expect("must solve");
        assert_eq!(1227775554, actual);
    }
    #[test]
    fn test_part_2() {
        let actual = solve_str(INPUT, part2).expect("must solve");
        assert_eq!(4174379265, actual);
    }
}
