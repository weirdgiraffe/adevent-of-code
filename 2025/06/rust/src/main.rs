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

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

fn parse<'a>(input: &'a str) -> (Vec<Op>, Vec<Vec<String>>) {
    let mut offest: Vec<usize> = Vec::new();

    let ops = input
        .lines()
        .rev()
        .take(1)
        .fold(Vec::<Op>::new(), |mut ops, line| {
            line.chars().enumerate().for_each(|(pos, ch)| {
                match ch {
                    '*' => {
                        offest.push(pos);
                        ops.push(Op::Mul);
                    }
                    '+' => {
                        offest.push(pos);
                        ops.push(Op::Add);
                    }
                    _ => {}
                };
            });
            ops
        });

    let mut nums = input
        .lines()
        .take_while(|line| !matches!(line.chars().nth(0).unwrap(), '*' | '+'))
        .fold(Vec::<Vec<String>>::new(), |mut nums, line| {
            let row = offest
                .iter()
                .zip(offest.iter().skip(1).chain([line.len() + 1].iter()))
                .fold(Vec::<String>::new(), |mut row, (&i, &j)| {
                    row.push(line[i..j - 1].to_owned());
                    row
                });
            nums.push(row);
            nums
        });

    nums = transpose(nums);

    // right pad elements
    nums.iter_mut().for_each(|row| {
        let pad = row.iter().map(|x| x.len()).max().unwrap();
        row.iter_mut().for_each(|x| {
            *x = format!("{:>width$}", x, width = pad);
        });
    });
    // println!("ops={:#?}\nrows={:#?}", ops, nums);
    (ops, nums)
}

fn transpose(v: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let rows = v.len();
    let cols = v[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| v[r][c].clone()).collect())
        .collect()
}

fn count<F>(input: &str, f: F) -> u64
where
    F: Fn(&Op, &Vec<String>) -> u64,
{
    let (ops, rows) = parse(input);
    (0..ops.len())
        .map(|i| {
            let result = f(&ops[i], &rows[i]);
            println!("op={:?} rows={:?} result={}", ops[i], rows[i], result);
            result
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    count(input, |op, args| {
        let nums = args
            .iter()
            .map(|x| x.trim().parse::<u64>().expect("must parse number"));
        match op {
            Op::Mul => nums.fold(1, |acc, x| acc * x),
            Op::Add => nums.sum(),
        }
    })
}

fn part2(input: &str) -> u64 {
    count(input, |op, args| {
        let nums = (0..args[0].len()).rev().map(|i| {
            args.iter()
                // .inspect(|x| println!("row={}", x))
                .flat_map(|line| {
                    let ch = line.chars().nth(i);
                    // println!("ch[{}]={:?}", i, ch);
                    ch
                })
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .expect("must parse number")
        });
        // .inspect(|x| println!("x={}", x));
        match op {
            Op::Mul => nums.fold(1, |acc, x| acc * x),
            Op::Add => nums.sum(),
        }
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
    use indoc::indoc;
    const INPUT: &str = indoc! {
        "
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
        "
    };

    #[test]
    fn test_part1() {
        let actual = solve_str(INPUT, part1);
        assert_eq!(4277556, actual);
    }
    #[test]
    fn test_part2() {
        let actual = solve_str(INPUT, part2);
        assert_eq!(3263827, actual);
    }
}

// wrong: 8907711267206
//        8907730960817
