use anyhow::{Context, Error, Result, anyhow};
use num_format::{CustomFormat, Grouping, ToFormattedString};
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

fn solve_file<F>(path: &str, f: F) -> Result<usize>
where
    F: Fn(&str) -> usize,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(solve_str(&content, f))
}

fn solve_str<F>(content: &str, f: F) -> usize
where
    F: Fn(&str) -> usize,
{
    f(content)
}

#[derive(Debug, Clone)]
struct Range {
    beg: usize,
    end: usize,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = CustomFormat::builder()
            .grouping(Grouping::Standard)
            .separator("_")
            .build()
            .unwrap();
        write!(
            f,
            "[{}, {}]",
            self.beg.to_formatted_string(&format),
            self.end.to_formatted_string(&format),
        )
    }
}

impl Range {
    fn contains(&self, num: usize) -> bool {
        num >= self.beg && num <= self.end
    }

    fn items(&self) -> usize {
        if self.end == self.beg {
            1
        } else {
            self.end - self.beg + 1
        }
    }
}

impl TryFrom<&str> for Range {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split("-").collect::<Vec<_>>();
        let beg: usize = parts
            .get(0)
            .ok_or(anyhow!("{}: not enough items for range", s))?
            .parse()?;
        let end: usize = parts
            .get(1)
            .ok_or(anyhow!("{}: not enough items for range", s))?
            .parse()?;
        Ok(Self { beg, end })
    }
}

fn part1(input: &str) -> usize {
    let ranges: Vec<Range> = input
        .lines()
        .take_while(|line| line.len() > 0)
        .map(|s| s.try_into())
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse ranges");
    let ingridients: Vec<usize> = input
        .lines()
        .skip(ranges.len() + 1)
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse ingridients");

    ingridients
        .iter()
        .flat_map(|i| ranges.iter().map(|r| r.contains(*i)).find(|&x| x))
        .filter(|&x| x)
        .count()
}

fn compact(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.beg);

    let mut result = Vec::with_capacity(ranges.len());
    let mut current = ranges[0].to_owned();

    for r in ranges.into_iter().skip(1) {
        if r.beg <= current.end.saturating_add(1) {
            // If `r` overlaps or touches `current`, merge them.
            if r.end > current.end {
                current.end = r.end;
            }
        } else {
            result.push(current);
            current = r.to_owned();
        }
    }
    result.push(current);
    result
}

fn part2(input: &str) -> usize {
    let ranges: Vec<Range> = compact(
        input
            .lines()
            .take_while(|line| line.len() > 0)
            .map(|s| s.try_into())
            .collect::<Result<Vec<Range>, _>>()
            .expect("must collect ranges"),
    );
    ranges.iter().map(|r| r.items()).sum()
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
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
        "
    };

    #[test]
    fn test_part1() {
        let actual = solve_str(INPUT, part1);
        assert_eq!(3, actual);
    }

    #[test]
    fn test_part2() {
        let actual = solve_str(INPUT, part2);
        assert_eq!(14, actual);
    }
}
