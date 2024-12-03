use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    struct MulCall {
        left: i32,
        right: i32,
    }

    impl MulCall {
        fn from(l_str: &str, r_str: &str) -> Result<Self> {
            Ok(Self {
                left: l_str.parse()?,
                right: r_str.parse()?,
            })
        }

        fn call(self) -> i32 {
            self.left * self.right
        }
    }

    enum Operation {
        Do,
        Dont,
        Mul(MulCall),
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let v = reader
            .lines()
            .flatten()
            .flat_map(|line| {
                mul_re
                    .captures_iter(&line)
                    .map(|c| c.extract())
                    .map(|(_, [l_str, r_str])| MulCall::from(l_str, r_str).unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|mul| mul.call())
            .sum();
        Ok(v)
    }

    assert_eq!(161, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let op_re = Regex::new(r"(do(?:n't)?)\(()()\)|(mul)\((\d+),(\d+)\)").unwrap();
        let operations = reader.lines().flatten().flat_map(|line| {
            op_re
                .captures_iter(&line)
                .map(|c| c.extract())
                .map(|(_, [operation, l_str, r_str])| match operation {
                    "do" => Operation::Do,
                    "don't" => Operation::Dont,
                    "mul" => Operation::Mul(MulCall::from(l_str, r_str).unwrap()),
                    _ => panic!("Unknown operation found"),
                })
                .collect::<Vec<_>>()
        });

        enum OperationResult {
            Enabled(i32),
            Disabled(i32),
        }

        impl From<OperationResult> for i32 {
            fn from(value: OperationResult) -> Self {
                match value {
                    OperationResult::Enabled(v) => v,
                    OperationResult::Disabled(v) => v,
                }
            }
        }

        let sum = operations
            .fold(OperationResult::Enabled(0), |result, operation| match operation {
                Operation::Do => OperationResult::Enabled(result.into()),
                Operation::Dont => OperationResult::Disabled(result.into()),
                Operation::Mul(mul_call) => match result {
                    OperationResult::Enabled(v) => OperationResult::Enabled(v + mul_call.call()),
                    OperationResult::Disabled(_) => result,
                },
            })
            .into();

        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
