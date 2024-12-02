use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(PartialEq, Debug)]
    enum ChangeType {
        Increasing,
        Decreasing,
    }

    fn get_diff_between_elements(parts: &Vec<i32>, a: usize, b: usize) -> Option<i32> {
        Some(parts.get(b)? - parts.get(a)?)
    }

    fn get_change_type(diff: i32) -> Option<ChangeType> {
        if diff < 0 {
            Some(ChangeType::Decreasing)
        } else if diff > 0 {
            Some(ChangeType::Increasing)
        } else {
            None
        }
    }

    fn parse_levels_from_report(line: &String) -> Vec<i32> {
        line.split(' ')
            .map(|level| i32::from_str_radix(level, 10).unwrap())
            .collect()
    }

    fn report_is_safe(levels: &Vec<i32>) -> bool {
        let change_type = match get_change_type(get_diff_between_elements(&levels, 0, 1).unwrap()) {
            Some(x) => x,
            None => return false,
        };

        for i in (Range {
            start: 0,
            end: levels.len() - 1,
        }) {
            let diff_to_next = get_diff_between_elements(&levels, i, i + 1).unwrap();

            match get_change_type(diff_to_next) {
                Some(this_change) => {
                    if this_change != change_type {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }

            let abs_diff = diff_to_next.unsigned_abs();
            if !(abs_diff >= 1 && abs_diff <= 3) {
                return false;
            }
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader
            .lines()
            .flatten()
            .filter(|line| report_is_safe(&parse_levels_from_report(line)));

        Ok(lines.count())
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn dropped_level_variants(levels: Vec<i32>) -> Vec<Vec<i32>> {
        let mut variants = Vec::new();
        for i in (Range {
            start: 0,
            end: levels.len(),
        }) {
            let pre = &levels[..i];
            let post = &levels[i + 1..];
            variants.push(pre.iter().chain(post).cloned().collect_vec());
        }
        variants
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().filter(|line| {
            let levels = parse_levels_from_report(line);

            let is_safe_natively = report_is_safe(&levels);
            if is_safe_natively {
                return true;
            }

            let variants = dropped_level_variants(levels);

            let safe_reports = variants.into_iter().filter(|report| report_is_safe(report));

            safe_reports.count() >= 1
        });

        Ok(lines.count())
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
