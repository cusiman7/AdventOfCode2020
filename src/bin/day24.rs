
use aoc::Vec3;
use std::collections::HashMap;
use self::HexDir::*;

#[derive(Debug)]
enum HexDir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl HexDir {
    // https://www.redblobgames.com/grids/hexagons/#neighbors-cube
    fn cube_dir(&self) -> Vec3 {
        match *self {
            E  => Vec3{x:1, y:-1, z: 0},
            SE => Vec3{x:0, y:-1, z:1},
            SW => Vec3{x:-1, y:0, z:1},
            W  => Vec3{x:-1, y:1, z:0},
            NW => Vec3{x:0, y:1, z:-1},
            NE => Vec3{x:1, y:0, z:-1},
        }
    }

    fn from_str(s: &'_ str) -> (HexDir, &'_ str) {
        if s.starts_with("e") {
            return (E, &s[1..]);
        } else if s.starts_with("se") {
            return (SE, &s[2..]);
        } else if s.starts_with("sw") {
            return (SW, &s[2..]);
        } else if s.starts_with("w") {
            return (W, &s[1..]);
        } else if s.starts_with("nw") {
            return (NW, &s[2..]);
        } else if s.starts_with("ne") {
            return (NE, &s[2..]);
        }
        panic!("ruh roh");
    }

    fn iter() -> std::slice::Iter<'static, HexDir> {
        static DIRS: [HexDir; 6] = [E, SE, SW, W, NW, NE];
        DIRS.iter()
    }
}

fn move_to(mut s: &str) -> Vec3 {
    let mut pos: Vec3 = Vec3{x:0, y:0, z:0};
    while !s.is_empty() {
        let (dir, rest) = HexDir::from_str(s);
        s = rest;
        pos += dir.cube_dir();
    }
    pos
}

fn main() {
    let mut tiles: HashMap<Vec3, bool> = HashMap::new();

    for line in aoc::file_lines_iter("./day24.txt") {
        let pos = move_to(&line);
        let tile_value: &mut bool = tiles.entry(pos.clone()).or_insert(false);
        *tile_value = !*tile_value;
    }
    let part1 = tiles.values().filter(|&&is_black| is_black).count();
    println!("Part1: {}", part1);

    let mut next_tiles = tiles.clone();
    for _ in 0..100 {
        next_tiles = tiles.clone();
        // Expand regions around black tiles
        for (pos, _) in tiles.iter().filter(|(_, &is_black)| is_black) {
            for dir in HexDir::iter() {
                next_tiles.entry(*pos + dir.cube_dir()).or_insert(false);
            }
        }
        tiles = next_tiles.clone();

        // Apply rules
        for (pos, tile) in &tiles {
            let mut black_count = 0;
            for dir in HexDir::iter() {
                if *tiles.get(&(*pos + dir.cube_dir())).unwrap_or(&false) {
                    black_count += 1;
                }
            }
            if *tile && (black_count == 0 || black_count > 2) {
                *next_tiles.entry(*pos).or_insert(false) = false;
            } else if !*tile && black_count == 2 {
                *next_tiles.entry(*pos).or_insert(true) = true;
            }
        }
        tiles = next_tiles.clone();
    }

    let part2 = next_tiles.values().filter(|&&is_black| is_black).count();
    println!("Part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use aoc::Vec3;
    use crate::move_to;
    #[test]
    fn examples() {
        assert_eq!(move_to("esew"), Vec3{x:0, y:-1, z:1}); 
        assert_eq!(move_to("nwwswee"), Vec3{x:0, y:0, z:0}); 
    }
}

