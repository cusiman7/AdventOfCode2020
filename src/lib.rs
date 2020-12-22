
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;
use std::fmt::Debug;

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_vec<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/// Returns an Iterator to the lines of the file.
pub fn file_lines_iter<F>(file: F) -> impl Iterator<Item = String>
where F: AsRef<Path> {
    let f = File::open(file).expect("No such file");
    io::BufReader::new(f)
        .lines()
        .map(Result::unwrap)
}

/// Parse a line of csv separated Ts into a Iter<T> 
pub fn csv_iter<T: FromStr>(line: &str) -> impl Iterator<Item = T> + '_
where <T as FromStr>::Err: Debug {
    line.split(",").map(move |v| v.parse::<T>().expect(&line))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

