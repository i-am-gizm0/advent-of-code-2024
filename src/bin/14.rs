use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

#[derive(Debug)]
struct Robot {
    position: Coord,
    velocity: Delta,
}

impl Robot {
    fn simulate(&self, (width, height): (usize, usize)) -> Robot {
        let width = width.try_into().unwrap();
        let height = height.try_into().unwrap();
        let mut position = self.position + self.velocity;
        if position.x < 0 {
            position.x += width;
        }

        if position.x >= width {
            position.x -= width;
        }

        if position.y < 0 {
            position.y += height;
        }

        if position.y >= height {
            position.y -= height;
        }

        Robot {
            velocity: self.velocity,
            position,
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn debug_robots(robots: &Vec<Robot>, (width, height): (usize, usize)) {
        let mut positions: HashMap<Coord, usize> = HashMap::new();
        for robot in robots {
            let count = positions.get(&robot.position).unwrap_or(&0);
            positions.insert(robot.position, *count + 1);
        }

        for y in 0..height {
            for x in 0..width {
                let count = positions.get(&Coord {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
                print!(
                    "{}",
                    match count {
                        Some(count) => {
                            count.to_string()
                        }
                        None => String::from("."),
                    }
                );
            }
            println!();
        }
    }

    fn part1<R: BufRead>(reader: R, (width, height): (usize, usize)) -> Result<usize> {
        let mut robots = reader
            .lines()
            .flatten()
            .map(|line| {
                let (position, velocity) = line.split_once(' ').unwrap();
                let (x, y) = position[2..].split_once(',').unwrap();
                let (dx, dy) = velocity[2..].split_once(',').unwrap();
                Robot {
                    position: Coord {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    },
                    velocity: Delta {
                        x: dx.parse().unwrap(),
                        y: dy.parse().unwrap(),
                    },
                }
            })
            .collect_vec();

        for _second in 0..100 {
            robots = robots
                .iter()
                .map(|robot| robot.simulate((width, height)))
                .collect_vec();
        }
        debug_robots(&robots, (width, height));

        let mid_x = width / 2;
        let mid_y = height / 2;
        let safety_factor = robots
            .iter()
            .into_group_map_by(|robot| {
                let Coord { x, y } = robot.position;
                let x: usize = x.try_into().unwrap();
                let y: usize = y.try_into().unwrap();

                if y < mid_y {
                    if x < mid_x {
                        1
                    } else if x > mid_x {
                        2
                    } else {
                        0
                    }
                } else if y > mid_y {
                    if x < mid_x {
                        3
                    } else if x > mid_x {
                        4
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .iter()
            .filter(|(group, _)| **group != 0)
            .map(|(_, robots)| robots.len())
            .product();

        Ok(safety_factor)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), (11, 7))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, (101, 103))?);
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
