use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_int(val: &str) -> u32 {
        u32::from_str_radix(val, 10).unwrap()
    }

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let parsed_lines = reader.lines().flatten().map(|line| {
            let (left, right) = line.split_at(line.find("   ").unwrap());
            (parse_int(left), parse_int(&right[3..]))
        });
        let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = parsed_lines.unzip();
        left_list.sort();
        right_list.sort();
        let answer = left_list
            .iter()
            .zip(right_list)
            .map(|(left, right)| left.abs_diff(right))
            .sum();
        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

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
