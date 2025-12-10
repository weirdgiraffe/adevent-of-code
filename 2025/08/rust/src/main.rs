use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Read;

type ResultType = usize;

fn solve_file<F>(path: &str, f: F) -> Result<ResultType>
where
    F: Fn(&str) -> ResultType,
{
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(f(&content))
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_boxes(input: &str) -> Vec<Point> {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .fold(Vec::<Point>::new(), |mut acc, line| {
            let mut iter = line
                .split(",")
                .map(|s| s.parse().expect("must be a number"));
            acc.push(Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
                z: iter.next().unwrap(),
            });
            acc
        })
}

fn distance(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)).isqrt()
}

fn part1_for(n: usize) -> impl Fn(&str) -> ResultType {
    move |s| part1(s, n)
}

fn part1(input: &str, count: usize) -> usize {
    let boxes = parse_boxes(input);
    let mut distances = boxes
        .iter()
        .enumerate()
        .map(|(i, a)| {
            boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| ((i, j), distance(a, b)))
        })
        .flatten()
        .collect::<Vec<_>>();

    distances.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let mut circuits: Vec<(Vec<usize>, usize)> = boxes
        .iter()
        .enumerate()
        .map(|(i, _)| (vec![i], i))
        .collect();

    for (i, j) in distances.iter().take(count).map(|&((i, j), _)| (i, j)) {
        let mut a = circuits[i].1; // owner of i
        while circuits[a].1 != a {
            a = circuits[a].1;
        }
        let mut b = circuits[j].1; // owner of j
        while circuits[b].1 != b {
            b = circuits[b].1;
        }
        if a == b {
            continue;
        }
        let mut drained = circuits[b].0.drain(0..).collect();
        circuits[a].0.append(&mut drained);
        circuits[b].1 = a;
    }

    let mut sizes = circuits
        .iter()
        .filter(|x| x.0.len() > 0)
        .map(|x| x.0.len())
        .collect::<Vec<_>>();
    sizes.sort();
    sizes.dedup();
    sizes.iter().rev().take(3).product()
}

fn part2(input: &str) -> usize {
    let boxes = parse_boxes(input);
    let mut distances = boxes
        .iter()
        .enumerate()
        .map(|(i, a)| {
            boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| ((i, j), distance(a, b)))
        })
        .flatten()
        .collect::<Vec<_>>();

    distances.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let mut circuits: Vec<(Vec<usize>, usize)> = boxes
        .iter()
        .enumerate()
        .map(|(i, _)| (vec![i], i))
        .collect();

    let mut left = circuits.len();
    for (i, j) in distances.iter().map(|&((i, j), _)| (i, j)) {
        let mut a = circuits[i].1; // owner of i
        while circuits[a].1 != a {
            a = circuits[a].1;
        }
        let mut b = circuits[j].1; // owner of j
        while circuits[b].1 != b {
            b = circuits[b].1;
        }
        if a == b {
            continue;
        }
        let mut drained = circuits[b].0.drain(0..).collect();
        circuits[a].0.append(&mut drained);
        circuits[b].1 = a;

        left -= 1;
        if left == 1 {
            return (boxes[i].x * boxes[j].x) as usize;
        }
    }
    panic!("should connect all boxes");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).context("missing file path argument")?;
    match args.get(2).map(|s| s.as_str()).context("missing part")? {
        "part1" => {
            println!("solution: {:#?}", solve_file(path, part1_for(1000))?);
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
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689

        "
    };

    #[test]
    fn test_part1() {
        let actual = part1(INPUT, 10);
        assert_eq!(40, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(INPUT);
        assert_eq!(25272, actual);
    }
}
