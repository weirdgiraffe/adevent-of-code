use anyhow::{Context, Result, anyhow};
use std::env;
use std::fs::File;
use std::io::Read;

fn solve_file<F>(path: &str, advance: F) -> Result<i32>
where
    F: Fn(&i32, &i32) -> (i32, i32),
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    solve_str(&content, advance)
}

fn solve_str<F>(content: &str, advance: F) -> Result<i32>
where
    F: Fn(&i32, &i32) -> (i32, i32),
{
    let distances: Vec<i32> = content
        .lines()
        .map(|line| {
            let direction: i32 = match line.chars().nth(0).ok_or(anyhow!("empty string"))? {
                'L' => -1,
                'R' => 1,
                _ => anyhow::bail!("unexpected input {}", line),
            };
            let steps: i32 = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .context("invalid distance")?;
            Ok(direction * steps)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let clicks = distances
        .iter()
        .scan(50, |pos, distance| {
            let (next, clicks) = advance(pos, distance);
            *pos = next;
            Some(clicks)
        })
        .sum();
    Ok(clicks)
}

fn part1(pos: &i32, offt: &i32) -> (i32, i32) {
    let next = (100 + (pos + offt) % 100).abs() % 100;
    // println!(
    //     "pos={} offt={} next={} click={}",
    //     pos,
    //     offt,
    //     next,
    //     next == 0
    // );
    match next == 0 {
        true => (next, 1),
        false => (next, 0),
    }
}

fn part2(pos: &i32, offt: &i32) -> (i32, i32) {
    let diff = pos + offt;

    let mut clicks = diff.abs() / 100;
    if diff <= 0 && *pos != 0 {
        clicks += 1;
    }
    let next = (100 + diff % 100).abs() % 100;
    // println!("pos={} offt={} next={} click={}", pos, offt, next, clicks);
    (next, clicks)
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
    const INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_part_1() {
        let actual = solve_str(INPUT, part1).expect("must solve");
        assert_eq!(3, actual);
    }

    #[test]
    fn test_part_2() {
        let actual = solve_str(INPUT, part2).expect("must solve");
        assert_eq!(6, actual);
    }
}
