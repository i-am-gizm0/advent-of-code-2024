use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{chain, Itertools};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

// const TEST: &str = "\
// ..........
// ..........
// ..........
// ....a.....
// ........a.
// .....a....
// ..........
// ......A...
// ..........
// ..........
// ";

// const TEST: &str = "\
// T.........
// ...T......
// .T........
// ..........
// ..........
// ..........
// ..........
// ..........
// ..........
// ..........
// ";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse<R: BufRead>(reader: R) -> ((usize, usize), HashMap<char, Vec<Coord>>) {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let height = lines.len();
        let width = lines.first().unwrap().len();

        let antenna_positions = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, char)| match char {
                        '.' => None,
                        _ => Some((
                            char,
                            Coord {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                        )),
                    })
                    .collect::<Vec<_>>()
            })
            .into_group_map();

        ((width, height), antenna_positions)
    }

    fn within_bounds(Coord { x, y }: &Coord, (width, height): (usize, usize)) -> bool {
        *x >= 0 && *x < width.try_into().unwrap() && *y >= 0 && *y < height.try_into().unwrap()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (bounds, antenna_positions) = parse(reader);

        let antinodes = antenna_positions
            .iter()
            .flat_map(|(_k, v)| {
                let antinodes = v
                    .iter()
                    .tuple_combinations()
                    .flat_map(|(a, b)| {
                        let delta = *a - *b;
                        vec![*a + delta, *b - delta]
                    })
                    .filter(|coord| within_bounds(coord, bounds))
                    .unique();

                // println!("{}", _k);
                // debug_print_grid(antinodes.clone(), bounds);

                antinodes
            })
            .unique();

        Ok(antinodes.count())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (bounds, antenna_positions) = parse(reader);
        let (width, _) = bounds;

        let antinodes = antenna_positions
            .iter()
            .flat_map(|(_k, v)| {
                let antinodes =
                    v.iter()
                        .tuple_combinations()
                        .flat_map(|(a, b)| {
                            let delta = *a - *b;
                            let pos_delta = delta.pos();
                            let step = pos_delta.x.try_into().unwrap();
                            let pos_iter = (a.x..(width.try_into().unwrap()))
                                .step_by(step)
                                .enumerate()
                                .map(move |(i, _)| *a + pos_delta * i.try_into().unwrap());
                            let neg_iter =
                                (0..a.x).rev().step_by(step).enumerate().map(move |(i, _)| {
                                    *a - pos_delta * (i + 1).try_into().unwrap()
                                });
                            chain(pos_iter, neg_iter)
                        })
                        .filter(|coord| within_bounds(coord, bounds))
                        .unique();

                // println!("{}", _k);
                // debug_print_grid(antinodes.clone(), bounds);

                antinodes
            })
            .unique();

        Ok(antinodes.count())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
