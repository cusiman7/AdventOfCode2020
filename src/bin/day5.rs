
use std::ops::{Add, Div, Range};
use std::cmp::max;
use num::{Integer, NumCast};

trait Midpoint<T> { 
    fn midpoint(&self) -> T;
}

impl<T> Midpoint<T> for Range<T>
where T: Add<Output = T> + Div<Output = T> + Copy + Integer + NumCast {
    fn midpoint(&self) -> T {
        return (self.start + self.end) / NumCast::from(2).unwrap();
    }
}

fn main() {
    let mut max_id = 0;
    let mut seats = Vec::<bool>::new();
    seats.resize(1000, false);

    for line in aoc::read_lines_as_vec("./day5.txt") {
        let mut row = 0..128;
        let mut col = 0..8;
        for c in line.chars() {
            match c {
                'F' => row = row.start..row.midpoint(),
                'B' => row = row.midpoint()..row.end,
                'L' => col = col.start..col.midpoint(),
                'R' => col = col.midpoint()..col.end,
                _ => panic!("That's not allowed stahp"),
            };
        }
        let id = row.start * 8 + col.start;
        seats[id] = true;
        max_id = max(max_id, id);
    }
    println!("Max ID: {}", max_id);
    for (i, seat) in seats.iter().enumerate() {
        if i == 0 { continue; }
        if i == max_id { break; }
        if seats[i-1] && !seat && seats[i+1] {
            println!("My Seat: {}", i);
            break;
        }
    }
}
