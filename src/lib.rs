
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Iterator;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
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

pub fn file_lines_iter<F>(file: F) -> impl Iterator<Item = String>
where F: AsRef<Path> {
    let f = File::open(file).expect("No such file");
    io::BufReader::new(f)
        .lines()
        .map(Result::unwrap)
}

