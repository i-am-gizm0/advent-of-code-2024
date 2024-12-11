use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> (Vec<Coord>, Vec<Vec<u32>>) {
        let mut trailheads: Vec<Coord> = Vec::new();
        let map = reader
            .lines()
            .flatten()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let height = char.to_digit(10).unwrap();
                        if height == 0 {
                            trailheads.push(Coord {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            });
                        }
                        height
                    })
                    .collect_vec()
            })
            .collect_vec();
        (trailheads, map)
    }

    fn neighbors(here: &Coord, map: &Vec<Vec<u32>>) -> Vec<Coord> {
        let mut neighbors = Vec::new();

        let on_left_edge = here.x == 0;
        let on_right_edge = here.x == (map.first().unwrap().len() - 1).try_into().unwrap();

        let on_top_edge = here.y == 0;
        let on_bottom_edge = here.y == (map.len() - 1).try_into().unwrap();

        if !on_left_edge {
            neighbors.push(Coord {
                x: here.x - 1,
                y: here.y,
            });
        }

        if !on_right_edge {
            neighbors.push(Coord {
                x: here.x + 1,
                y: here.y,
            });
        }

        if !on_top_edge {
            neighbors.push(Coord {
                x: here.x,
                y: here.y - 1,
            });
        }

        if !on_bottom_edge {
            neighbors.push(Coord {
                x: here.x,
                y: here.y + 1,
            });
        }

        neighbors
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (trailheads, map) = parse(reader);
        
        fn unique_reachable_peaks(here: &Coord, map: &Vec<Vec<u32>>) -> Vec<Coord> {
            let here_elevation = map[*here];
            neighbors(here, map)
                .iter()
                .flat_map(|neighbor| {
                    let neighbor_elevation = map[*neighbor];
                    if neighbor_elevation != here_elevation + 1 {
                        return vec![];
                    }
                    if neighbor_elevation == 9 {
                        return vec![neighbor.to_owned()];
                    }

                    unique_reachable_peaks(neighbor, map)
                })
                .unique()
                .collect_vec()
        }

        Ok(trailheads
            .iter()
            .flat_map(|trailhead| unique_reachable_peaks(trailhead, &map))
            .count())
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (trailheads, map) = parse(reader);

        fn unique_trails(here: &Coord, map: &Vec<Vec<u32>>) -> usize {
            let here_elevation = map[*here];
            neighbors(here, map)
                .iter()
                .map(|neighbor| {
                    let neighbor_elevation = map[*neighbor];
                    if neighbor_elevation != here_elevation + 1 {
                        return 0;
                    }
                    if neighbor_elevation == 9 {
                        return 1;
                    }

                    unique_trails(neighbor, map)
                }).sum()
        }

        Ok(trailheads
            .iter()
            .map(|trailhead| unique_trails(trailhead, &map))
            .sum())
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
