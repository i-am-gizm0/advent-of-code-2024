use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
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

    fn parse_int(val: &str) -> usize {
        usize::from_str_radix(val, 10).unwrap()
    }

    fn parse_file<R: BufRead>(reader: R) -> (Vec<usize>, Vec<usize>) {
        reader
            .lines()
            .flatten()
            .map(|line| {
                let (left, right) = line.split_at(line.find("   ").unwrap());
                (parse_int(left), parse_int(&right[3..]))
            })
            .unzip()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left_list, mut right_list) = parse_file(reader);
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left_list, right_list) = parse_file(reader);
        let mut similarity_score = 0;
        let mut right_counts: HashMap<usize, usize> = HashMap::new();

        for element in right_list {
            right_counts.insert(element, *right_counts.get(&element).unwrap_or(&0) + 1);
        }

        for element in left_list {
            similarity_score += right_counts.get(&element).unwrap_or(&0) * element;
        }

        Ok(similarity_score)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
