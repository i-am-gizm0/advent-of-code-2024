use std::{
    collections::HashSet,
    ops::{Add, Neg, Sub},
};

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}

// My Utilities
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

// impl PartialEq for Coord {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

// impl Eq for Coord {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Delta {
    pub x: isize,
    pub y: isize,
}

impl Neg for Delta {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub<Self> for Coord {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Delta> for Coord {
    type Output = Self;

    fn add(self, rhs: Delta) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Delta> for Coord {
    type Output = Self;

    fn sub(self, rhs: Delta) -> Self::Output {
        self + -rhs
    }
}

pub fn debug_print_grid(points: impl Iterator<Item = Coord>, size: (usize, usize)) -> () {
    let point_set: HashSet<Coord> = HashSet::from_iter(points);
    for y in 0..size.1 {
        for x in 0..size.0 {
            print!(
                "{}",
                if point_set.contains(&Coord {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap()
                }) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}
