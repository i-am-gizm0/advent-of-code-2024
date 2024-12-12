use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
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

    fn parse<R: BufRead>(reader: R) -> HashMap<usize, usize> {
        HashMap::from_iter(
            reader
                .lines()
                .flatten()
                .join(" ")
                .split(' ')
                .map(|val| (val.parse::<usize>().unwrap(), 1))
                .into_group_map()
                .iter()
                .map(|(k, values)| (*k, values.iter().sum())),
        )
    }

    fn add_instances_to_key(key: usize, count: usize, map: &mut HashMap<usize, usize>) {
        let current = *map.get(&key).unwrap_or(&0);
        map.insert(key, current + count);
    }

    fn blink(value_count: HashMap<usize, usize>) -> HashMap<usize, usize> {
        let stone_values: Vec<_> = value_count.keys().cloned().collect();

        let mut new_count_this_iter: HashMap<usize, usize> = HashMap::new();

        for stone in stone_values {
            let count = value_count[&stone];

            if stone == 0 {
                add_instances_to_key(1, count, &mut new_count_this_iter);
            } else {
                let stone_log = stone.ilog10();
                if stone_log % 2 == 1 {
                    let split_point = 10usize.pow(stone_log.div_ceil(2));

                    let left = stone / split_point;
                    let right = stone % split_point;

                    add_instances_to_key(left, count, &mut new_count_this_iter);
                    add_instances_to_key(right, count, &mut new_count_this_iter);
                } else {
                    add_instances_to_key(stone * 2024, count, &mut new_count_this_iter);
                }
            }
        }

        new_count_this_iter
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut value_count = parse(reader);

        for _i in 0..25 {
            // println!("Blink {}: {:?}", _i + 1, value_count);
            value_count = blink(value_count);
        }
        // println!("End: {:?}", value_count);

        Ok(value_count.values().sum())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut value_count = parse(reader);

        for _i in 0..75 {
            // println!("Blink {}: {:?}", _i + 1, value_count);
            value_count = blink(value_count);
        }
        // println!("End: {:?}", value_count);

        Ok(value_count.values().sum())
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
