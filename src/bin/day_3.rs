use derive_more::{Add, Sub};
use std::{num, str::FromStr};
use thiserror::Error;

const INPUT: &str = include_str!("day_3_input");

fn main() {
    let wires: Vec<_> = INPUT
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .map(|m| Movement::from_str(m).unwrap())
                .collect()
        })
        .map(|l: Vec<_>| Line::from(l))
        .collect();

    let intersections = wires[0].get_intersections(&wires[1]);

    let distances: Vec<_> = intersections
        .iter()
        .map(|intersection| intersection.distance_to(Coordinate(0, 0)))
        .collect();

    let smallest_distance = smallest(&distances).unwrap();
    println!("{}", smallest_distance);
}

fn abs(n: i64) -> i64 {
    if n < 0 {
        -n
    } else {
        n
    }
}

#[cfg(test)]
mod abs_tests {
    use super::*;

    #[test]
    fn pos() {
        assert_eq!(abs(1), 1);
        assert_eq!(abs(100), 100);
    }

    #[test]
    fn neg() {
        assert_eq!(abs(-1), 1);
        assert_eq!(abs(-100), 100);
    }

    #[test]
    fn zero() {
        assert_eq!(abs(0), 0);
    }
}

fn smallest<T: Copy + PartialOrd>(items: &[T]) -> Option<T> {
    items.into_iter().fold(None, |min, x| match min {
        None => Some(*x),
        Some(y) => Some(if y < *x { y } else { *x }),
    })
}

#[cfg(test)]
mod smallest_tests {
    use super::*;

    #[test]
    fn short() {
        assert_eq!(Some(10), smallest(&[100, 50, 75, 10]));
    }

    #[test]
    fn long() {
        assert_eq!(Some(-10), smallest(&[-10, 9, -7, -4, 7, 3, 1, 10]));
    }
}

// Just using .dedup() only removes duplicates that are adjacent to each other.
fn full_dedup<T: PartialEq + Ord>(items: &mut Vec<T>) {
    items.sort();
    items.dedup();
}

fn get_dups<T: Copy + PartialEq + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    let mut dups: Vec<_> = left
        .into_iter()
        .filter(|i| right.contains(i))
        .map(|i| *i)
        .collect();

    full_dedup(&mut dups);

    dups
}

#[cfg(test)]
mod dups_tests {
    use super::*;

    #[test]
    fn remove() {
        assert_eq!(vec![1, 2, 3], {
            let mut nums = vec![1, 2, 1, 3, 3, 2];
            full_dedup(&mut nums);
            nums
        });
    }

    #[test]
    fn get() {
        assert_eq!(
            vec![1, 5, 10],
            get_dups(&[1, 2, 3, 5, 7, 10, 10], &[5, 1, 8, 10])
        )
    }
}

#[derive(Add, Sub, Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Coordinate(i64, i64);

impl Coordinate {
    // Manhattan distance
    fn distance_to(self, other: Self) -> i64 {
        let mut difference = self - other;
        difference.0 = abs(difference.0);
        difference.1 = abs(difference.1);

        difference.0 + difference.1
    }
}

#[cfg(test)]
mod coordinate_tests {
    use super::*;

    #[test]
    fn distance_to_coordinate() {
        assert_eq!(Coordinate(0, 0).distance_to(Coordinate(10, 10)), 20);
        assert_eq!(Coordinate(-10, 5).distance_to(Coordinate(20, 0)), 35);
        assert_eq!(Coordinate(-50, 25).distance_to(Coordinate(0, 0)), 75);
    }
}

#[derive(Debug, PartialEq)]
enum Movement {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

#[derive(Error, Debug, PartialEq)]
enum MovementParseError {
    #[error("could not parse out amount")]
    AmountError(#[from] num::ParseIntError),
    #[error("could not parse out direction")]
    DirectionError,
}

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_amount = |c| !(char::is_numeric(c) || c == '-');
        let is_not_amount = |c| !is_amount(c);

        let amount: i64 = s.trim_matches(is_amount).parse()?;

        Ok(match s.trim_matches(is_not_amount) {
            "U" => Movement::Up(amount),
            "D" => Movement::Down(amount),
            "L" => Movement::Left(amount),
            "R" => Movement::Right(amount),
            _ => return Err(MovementParseError::DirectionError),
        })
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    #[test]
    fn movement_from_str() {
        assert_eq!(Movement::from_str("U500"), Ok(Movement::Up(500)));
        assert_eq!(Movement::from_str("D-100"), Ok(Movement::Down(-100)));
        assert_eq!(Movement::from_str("L10"), Ok(Movement::Left(10)));
        assert_eq!(Movement::from_str("R2"), Ok(Movement::Right(2)));
    }
}

#[derive(Debug, PartialEq)]
struct Line(Vec<Coordinate>);

impl From<Vec<Movement>> for Line {
    fn from(movements: Vec<Movement>) -> Self {
        // Lines are assumed to start at the origin.
        let mut coordinates = vec![Coordinate(0, 0)];

        movements.iter().for_each(|movement| {
            let (&total_offset, step) = match movement {
                Movement::Up(x) => (x, Coordinate(0, 1)),
                Movement::Down(x) => (x, Coordinate(0, -1)),
                Movement::Left(x) => (x, Coordinate(-1, 0)),
                Movement::Right(x) => (x, Coordinate(1, 0)),
            };

            // Create one coordinate for each step that a move is made up of.
            (0..total_offset).for_each(|_| coordinates.push(step + *coordinates.last().unwrap()));
        });

        // Lines don’t actually include the origin, however.
        coordinates.remove(0);

        Line(coordinates)
    }
}

impl Line {
    fn get_intersections(&self, other: &Self) -> Vec<Coordinate> {
        get_dups(&self.0, &other.0)
    }
}

#[cfg(test)]
mod line_tests {
    use super::*;

    fn gen_intersection_test(a: &[(i64, i64)], b: &[(i64, i64)], intersections: Vec<Coordinate>) {
        assert_eq!(
            Line(a.iter().map(|(x, y)| Coordinate(*x, *y)).collect())
                .get_intersections(&Line(b.iter().map(|(x, y)| Coordinate(*x, *y)).collect())),
            intersections
        );
    }

    #[test]
    fn test_intersections() {
        gen_intersection_test(
            &[(0, 0), (0, 1), (0, 2)],
            &[(-1, 1), (0, 1), (1, 1)],
            vec![Coordinate(0, 1)],
        );
    }

    fn gen_from_vec_test(movements: Vec<Movement>, coords: &[(i64, i64)]) {
        assert_eq!(
            Line::from(movements),
            Line(coords.iter().map(|(x, y)| Coordinate(*x, *y)).collect())
        )
    }

    #[test]
    fn from_vec_simple() {
        gen_from_vec_test(
            vec![Movement::Up(3), Movement::Left(3)],
            &[(0, 1), (0, 2), (0, 3), (-1, 3), (-2, 3), (-3, 3)],
        );
    }

    #[test]
    fn from_vec_complex() {
        gen_from_vec_test(
            vec![Movement::Down(2), Movement::Right(1), Movement::Up(3)],
            &[(0, -1), (0, -2), (1, -2), (1, -1), (1, 0), (1, 1)],
        );
    }
}
