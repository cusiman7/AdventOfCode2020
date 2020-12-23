
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }
}

impl Mul<i32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

#[macro_export]
macro_rules! multi_for {
    ([$x:ident=$x_range:expr, $y:ident=$y_range:expr] $($body:tt)*) => {
        for $y in $y_range {
            for $x in $x_range {
                $($body)*
            }
        }
    };
    ([$x:ident=$x_range:expr, $y:ident=$y_range:expr, $z:ident=$z_range:expr] $($body:tt)*) => {
        for $z in $z_range {
            for $y in $y_range {
                for $x in $x_range {
                    $($body)*
                }
            }
        }
    };
    ([$x:ident=$x_range:expr, $y:ident=$y_range:expr, $z:ident=$z_range:expr, $w:ident=$w_range:expr] $($body:tt)*) => {
        for $w in $w_range {
            for $z in $z_range {
                for $y in $y_range {
                    for $x in $x_range {
                        $($body)*
                    }
                }
            }
        }
    };
}

