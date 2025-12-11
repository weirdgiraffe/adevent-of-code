use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;

type ResultType = usize;

fn solve_file<F>(path: &str, f: F) -> Result<ResultType>
where
    F: Fn(&str) -> ResultType,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(f(&content))
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Ord)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_points(input: &str) -> Vec<Point> {
    input.lines().take_while(|line| !line.is_empty()).fold(
        Vec::<Point>::new(),
        |mut floor, line| {
            let mut iter = line
                .split(",")
                .map(|s| s.parse().expect("must be a number"));
            floor.push(Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            });
            floor
        },
    )
}

fn part1(input: &str) -> ResultType {
    let points = parse_points(input);
    let mut rectangles = points
        .iter()
        .enumerate()
        .map(|(i, p1)| points.iter().skip(i + 1).map(move |p2| (p1, p2)))
        .flatten()
        .map(|(p1, p2)| {
            let a: usize = p1.x.abs_diff(p2.x) + 1;
            let b: usize = p1.y.abs_diff(p2.y) + 1;
            let area = a * b;
            println!("{:?} area {}", (p1, p2), area);
            area
        })
        .collect::<Vec<_>>();
    rectangles.sort();
    rectangles.last().unwrap().to_owned()
}

fn part2(input: &str) -> ResultType {
    // Parse points in their original order to preserve polygon structure
    let points = parse_points(input);
    points.iter().enumerate().fold(0usize, |max_area, (i, p1)| {
        points.iter().skip(i + 1).fold(max_area, |max_area, p2| {
            let xrange = p1.x.min(p2.x)..=p1.x.max(p2.x);
            let yrange = p1.y.min(p2.y)..=p1.y.max(p2.y);
            let area = (xrange.end() - xrange.start() + 1) * (yrange.end() - yrange.start() + 1);
            if area > max_area && is_inside(&points, xrange, yrange) {
                return max_area.max(area);
            }
            max_area
        })
    })
}

type Border = RangeInclusive<usize>;

fn is_inside(points: &Vec<Point>, xrange: Border, yrange: Border) -> bool {
    // We need to check if every border within points (vertical or horizontal) includes the
    // rectangle described by the provided xrange or y range.
    points
        .iter()
        .zip(points.iter().cycle().skip(1).take(points.len()))
        .all(|(p1, p2)| {
            if p1.y == p2.y {
                // check horizontal border p1.x..=p2.x
                return p1.y.cmp(yrange.end()).is_ge()
                    || p1.y.cmp(yrange.start()).is_le()
                    || (p1.x.cmp(xrange.start()).is_le() && p2.x.cmp(xrange.start()).is_le())
                    || (p1.x.cmp(xrange.end()).is_ge() && p2.x.cmp(xrange.end()).is_ge());
            }
            if p1.x == p2.x {
                // check vertical border p1.y..=p2.y
                return p1.x.cmp(xrange.end()).is_ge()
                    || p1.x.cmp(xrange.start()).is_le()
                    || (p1.y.cmp(yrange.start()).is_le() && p2.y.cmp(yrange.start()).is_le())
                    || (p1.y.cmp(yrange.end()).is_ge() && p2.y.cmp(yrange.end()).is_ge());
            }
            panic!("unexpected polygon shape: two points can not build a border");
        })
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).context("missing file path argument")?;
    match args.get(2).map(|s| s.as_str()).context("missing part")? {
        "part1" => {
            println!("solution: {:#?}", solve_file(path, part1)?);
        }
        "part2" => {
            // solution: 3161295996 - too high
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
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
        "
    };

    #[test]
    fn test_part1() {
        let actual = part1(INPUT);
        assert_eq!(50, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(INPUT);
        assert_eq!(24, actual);
    }
}
