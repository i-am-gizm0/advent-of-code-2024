use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn blink(stones: &mut Vec<usize>) -> () {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];

            if stone == 0 {
                stones[i] = 1;
            } else {
                let stringified = stone.to_string();
                if stringified.len() % 2 == 0 {
                    // Could also do stone.ilog10() % 2 == 0, but we need the string representation anyway
                    let (left, right) = stringified.split_at(stringified.len() / 2);
                    stones.splice(
                        i..i + 1,
                        vec![left.parse().unwrap(), right.parse().unwrap()],
                    );
                    i += 1; // Skip the stone we just created
                } else {
                    stones[i] = stone * 2024;
                }
            }

            i += 1;
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut stones: Vec<usize> = Vec::from_iter(
            reader
                .lines()
                .flatten()
                .join(" ")
                .split(' ')
                .map(|val| val.parse().unwrap()),
        );

        for _i in 0..25 {
            // println!("After {} blink(s): {:?}", _i, stones);

            blink(&mut stones);
        }

        // println!("After blinks: {:?}", stones);

        Ok(stones.len())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

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
