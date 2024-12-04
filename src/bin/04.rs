use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

/*
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

 */

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl<T> Index<Coord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, Coord { x, y }: Coord) -> &Self::Output {
        &self[y][x]
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let x_instances: Vec<Coord> = (&grid)
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(x, char)| match char {
                        &'X' => Some(Coord { x, y }),
                        _ => None,
                    })
            })
            .collect();

        enum Direction {
            Down,
            DownRight,
            Right,
            UpRight,
            Up,
            UpLeft,
            Left,
            DownLeft,
        }

        fn iter_direction(
            Coord { x: x_x, y: x_y }: &Coord,
            grid: &Vec<Vec<char>>,
            direction: &Direction,
        ) -> Option<String> {
            let x_mul = match direction {
                Direction::Down => 0,
                Direction::DownRight => 1,
                Direction::Right => 1,
                Direction::UpRight => 1,
                Direction::Up => 0,
                Direction::UpLeft => -1,
                Direction::Left => -1,
                Direction::DownLeft => -1,
            };
            let y_mul = match direction {
                Direction::Down => 1,
                Direction::DownRight => 1,
                Direction::Right => 0,
                Direction::UpRight => -1,
                Direction::Up => -1,
                Direction::UpLeft => -1,
                Direction::Left => 0,
                Direction::DownLeft => 1,
            };

            // https://stackoverflow.com/a/54035801
            fn add(u: usize, i: i32) -> Option<usize> {
                if i.is_negative() {
                    u.checked_sub(i.wrapping_abs() as u32 as usize)
                } else {
                    u.checked_add(i as usize)
                }
            }

            let coords = (0..4).map(|offset: i32| {
                let x = add(*x_x, offset * x_mul)?;
                let y = add(*x_y, offset * y_mul)?;
                Some(Coord { x, y })
            });
            let all_in_bounds = coords.to_owned().borrow_mut().all(|v| match v {
                Some(Coord { x, y }) => x < grid.len() && y < grid.first().unwrap().len(),
                None => false,
            });
            if !all_in_bounds {
                return None;
            }
            Some(String::from_iter(coords.map(|coord| grid[coord.unwrap()])))
        }

        Ok(x_instances
            .iter()
            .map(|coord| {
                [
                    Direction::Down,
                    Direction::DownRight,
                    Direction::Right,
                    Direction::UpRight,
                    Direction::Up,
                    Direction::UpLeft,
                    Direction::Left,
                    Direction::DownLeft,
                ]
                .iter()
                .filter_map(|direction| iter_direction(&coord, &grid, direction))
                .filter(|str| str == &"XMAS")
                .count()
            })
            .sum())
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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
