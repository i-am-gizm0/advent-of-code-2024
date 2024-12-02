use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().filter_map(|line| {
            let parts: Vec<_> = line
                .split(' ')
                .map(|level| i32::from_str_radix(level, 10).unwrap())
                .collect();

            let change_type = get_change_type(get_diff_between_elements(&parts, 0, 1).unwrap())?;

            for i in (Range {
                start: 0,
                end: parts.len() - 1,
            }) {
                let diff_to_next = get_diff_between_elements(&parts, i, i + 1).unwrap();
                if get_change_type(diff_to_next)? != change_type {
                    return None;
                }

                let abs_diff = diff_to_next.unsigned_abs();
                if !(abs_diff >= 1 && abs_diff <= 3) {
                    return None;
                }
            }

            Some(parts)
        });

        Ok(lines.count())
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().filter_map(|line| {
            let parts: Vec<_> = line
                .split(' ')
                .map(|level| i32::from_str_radix(level, 10).unwrap())
                .collect();

            let change_type =
                get_change_type(get_diff_between_elements(&parts, 0, 1).unwrap()).unwrap();

            println!("change type: {:?}", change_type);

            let mut unsafe_levels: u32 = 0;

            for i in (Range {
                start: 0,
                end: parts.len() - 1,
            }) {
                print!(
                    "({}, {})\t",
                    parts.get(i).unwrap(),
                    parts.get(i + 1).unwrap()
                );
                let diff_to_next = get_diff_between_elements(&parts, i, i + 1).unwrap();
                if get_change_type(diff_to_next)
                    .map_or(true, |this_change| this_change != change_type)
                {
                    println!("unsafe change type");
                    unsafe_levels += 1;
                    continue;
                }

                let abs_diff = diff_to_next.unsigned_abs();
                if !(abs_diff >= 1 && abs_diff <= 3) {
                    println!("unsafe difference");
                    unsafe_levels += 1;
                    continue;
                }

                println!("safe!");
            }

            let dampened_safe = unsafe_levels <= 1;

            println!(
                "{:?} is {} ({} unsafe levels)",
                parts,
                if dampened_safe { "safe" } else { "unsafe" },
                unsafe_levels
            );

            if dampened_safe {
                Some(parts)
            } else {
                None
            }
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
