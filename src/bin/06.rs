use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(std::fmt::Debug, Clone, Copy)]
enum GuardPose {
    Up(Coord),
    Right(Coord),
    Down(Coord),
    Left(Coord),
}

impl GuardPose {
    fn from(facing: &char, coord: Coord) -> Option<Self> {
        match *facing {
            '^' => Some(Self::Up(coord)),
            '>' => Some(Self::Right(coord)),
            'v' => Some(Self::Down(coord)),
            '<' => Some(Self::Left(coord)),
            _ => None,
        }
    }

    fn see_forward(self) -> Coord {
        match self {
            GuardPose::Up(Coord { x, y }) => Coord { x, y: y - 1 },
            GuardPose::Right(Coord { x, y }) => Coord { x: x + 1, y },
            GuardPose::Down(Coord { x, y }) => Coord { x, y: y + 1 },
            GuardPose::Left(Coord { x, y }) => Coord { x: x - 1, y },
        }
    }

    fn move_forward(self) -> Self {
        let coord = self.see_forward();
        match self {
            GuardPose::Up(_) => GuardPose::Up(coord),
            GuardPose::Right(_) => GuardPose::Right(coord),
            GuardPose::Down(_) => GuardPose::Down(coord),
            GuardPose::Left(_) => GuardPose::Left(coord),
        }
    }

    fn position(self) -> Coord {
        match self {
            GuardPose::Up(coord) => coord,
            GuardPose::Right(coord) => coord,
            GuardPose::Down(coord) => coord,
            GuardPose::Left(coord) => coord,
        }
    }

    fn rotate(self) -> Self {
        match self {
            GuardPose::Up(coord) => Self::Right(coord),
            GuardPose::Right(coord) => Self::Down(coord),
            GuardPose::Down(coord) => Self::Left(coord),
            GuardPose::Left(coord) => Self::Up(coord),
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (guard, obstacles, size) = reader.lines().flatten().enumerate().fold(
            (None, HashSet::new(), (0, 0)),
            |(guard, mut obstacles, (old_width, _)), (y, line)| {
                let mut guard = guard;
                let chars: Vec<_> = line.chars().collect();
                let width = chars.len();
                if y != 0 && width != old_width {
                    panic!("Non-rectangular areas are not supported!");
                }
                chars.iter().enumerate().for_each(|(x, chr)| {
                    let coord = Coord {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    if chr == &'#' {
                        obstacles.insert(coord);
                    } else {
                        match GuardPose::from(chr, coord) {
                            Some(pose) => match guard {
                                Some(_) => panic!("Multiple guards!"),
                                None => {
                                    guard = Some(pose);
                                }
                            },
                            None => {}
                        }
                    }
                });
                (guard, obstacles, (width, y + 1))
            },
        );

        let mut guard = match guard {
            Some(pose) => pose,
            None => panic!("No guard in input!"),
        };

        fn within(guard: &GuardPose, (width, height): &(usize, usize)) -> bool {
            let position: Coord = guard.position();
            position.x >= 0
                && position.x < (*width).try_into().unwrap()
                && position.y >= 0
                && position.y < (*height).try_into().unwrap()
        }

        let mut visited = HashSet::new();

        while within(&guard, &size) {
            visited.insert(guard.position());
            let ahead = guard.see_forward();
            if obstacles.contains(&ahead) {
                guard = guard.rotate();
            }
            guard = guard.move_forward();
        }

        Ok(visited.len())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
