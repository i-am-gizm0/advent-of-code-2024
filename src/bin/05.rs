use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
        let (rules, mut updates): (VecDeque<_>, VecDeque<_>) = reader
            .lines()
            .flatten()
            .partition(|line| line.contains('|'));

        let must_be_before = rules
            .iter()
            .map(|rule| -> (usize, usize) {
                let (val, goes_before) = rule.split_once('|').unwrap();
                (val.parse().unwrap(), goes_before.parse().unwrap())
            })
            .into_group_map();

        updates.pop_front();

        let updates = updates
            .iter()
            .map(|update| update.split(',').map(|val| val.parse().unwrap()).collect())
            .collect();

        (must_be_before, updates)
    }

    fn disallowed_seen(
        page: &usize,
        seen: &HashSet<usize>,
        rules: &HashMap<usize, Vec<usize>>,
    ) -> bool {
        let this_page_rule = rules.get(page);
        match this_page_rule {
            Some(disallowed) => disallowed
                .iter()
                .any(|disallowed_val| seen.contains(disallowed_val)),
            None => false,
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (rules, updates) = parse(reader);

        Ok(updates
            .iter()
            .filter_map(|update| -> Option<usize> {
                let mut seen: HashSet<usize> = HashSet::new();
                for page in update {
                    if disallowed_seen(page, &seen, &rules) {
                        return None;
                    }
                    seen.insert(*page);
                }
                // This update is correctly ordered!
                Some(update[update.len() / 2])
            })
            .sum())
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

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
