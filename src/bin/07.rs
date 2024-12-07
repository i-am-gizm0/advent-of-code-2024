use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{chain, Itertools};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

struct Equation {
    test_val: usize,
    numbers: Vec<usize>,
}

#[derive(Clone, Debug)]
enum Operation {
    Identity,
    Multiply,
    Addition,
    Concatenation,
}

impl Operation {
    fn calc<
        N: std::ops::Mul<Output = N> + std::ops::Add<Output = N> + std::str::FromStr + ToString,
    >(
        self,
        a: N,
        b: N,
    ) -> N
    where
        <N as std::str::FromStr>::Err: std::fmt::Debug,
    {
        match self {
            Operation::Identity => b,
            Operation::Multiply => a * b,
            Operation::Addition => a + b,
            Operation::Concatenation => String::from_iter([a.to_string(), b.to_string()])
                .parse()
                .unwrap(),
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> impl Iterator<Item = Equation> {
        reader.lines().flatten().map(|line| {
            let (test_val, numbers) = line.split_once(": ").unwrap();

            Equation {
                test_val: test_val.parse().unwrap(),
                numbers: numbers.split(' ').map(|num| num.parse().unwrap()).collect(),
            }
        })
    }

    fn do_calculation(
        equations: impl Iterator<Item = Equation>,
        operations: Vec<Operation>,
    ) -> usize {
        equations
            .filter(|Equation { test_val, numbers }| {
                let operator_count = numbers.len() - 1;
                let mut multi_prod = (0..operator_count)
                    .map(|_| &operations)
                    .multi_cartesian_product()
                    .map(|v| (chain![&[Operation::Identity], v]));

                multi_prod.any(|operations| {
                    let val = numbers
                        .iter()
                        .zip(operations)
                        .fold(0, |a, (b, op)| op.to_owned().calc(a, b.to_owned()));
                    &val == test_val
                })
            })
            .map(
                |Equation {
                     test_val,
                     numbers: _,
                 }| test_val,
            )
            .sum()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let equations = parse(reader);

        Ok(do_calculation(
            equations,
            vec![Operation::Multiply, Operation::Addition],
        ))
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let equations = parse(reader);

        Ok(do_calculation(
            equations,
            vec![
                Operation::Multiply,
                Operation::Addition,
                Operation::Concatenation,
            ],
        ))
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
