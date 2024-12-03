use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    struct MulCall {
        left: i32,
        right: i32,
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let v = reader
            .lines()
            .flatten()
            .flat_map(|line| {
                let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                mul_re
                    .captures_iter(&line)
                    .map(|c| c.extract())
                    .map(|(_, [l_str, r_str])| MulCall {
                        left: l_str.parse().unwrap(),
                        right: r_str.parse().unwrap(),
                    })
                    .collect::<Vec<_>>()
            })
            .map(|mul| mul.left * mul.right)
            .sum();
        Ok(v)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
