
use std::collections::HashMap;
use aoc::{Vec3, Vec4, multi_for};

#[derive(Debug, Clone, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

type PocketDimension3D = HashMap<Vec3, Cube>;
type PocketDimension4D = HashMap<Vec4, Cube>;

fn dirs_3d() -> impl Iterator<Item = &'static Vec3> {
    static mut DIRS : Option<Vec<Vec3>> = None;
    // This whole thing is unsafe because static stuff could be mutated across multiple threads
    unsafe {
        if DIRS.is_none() {
            DIRS = Some(Vec::new());
            multi_for! { [x=-1..=1, y=-1..=1, z=-1..=1]
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                DIRS.as_mut().unwrap().push(Vec3{x:x, y:y, z:z});
            }
            assert!(DIRS.as_ref().unwrap().len() == 26);
        }
        DIRS.as_ref().unwrap().iter()
    }
}

fn dirs_4d() -> impl Iterator<Item = &'static Vec4> {
    static mut DIRS : Option<Vec<Vec4>> = None;
    // This whole thing is unsafe because static stuff could be mutated across multiple threads
    unsafe {
        if DIRS.is_none() {
            DIRS = Some(Vec::new());
            multi_for! { [x=-1..=1, y=-1..=1, z=-1..=1, w=-1..=1]
                if x == 0 && y == 0 && z == 0 && w == 0 {
                    continue;
                }
                DIRS.as_mut().unwrap().push(Vec4{x:x, y:y, z:z, w:w});
            }
            assert!(DIRS.as_ref().unwrap().len() == 80);
        }
        DIRS.as_ref().unwrap().iter()
    }
}

fn main() {
    let mut pocket_dim_3d = PocketDimension3D::new();
    let mut pocket_dim_4d = PocketDimension4D::new();
    let mut p = Vec3::default();
    for line in aoc::file_lines_iter("./day17.txt") {
        for c in line.as_bytes() {
            match c {
                b'.' => {
                    pocket_dim_3d.insert(p.clone(), Cube::Inactive);
                    pocket_dim_4d.insert(Vec4{x:p.x, y:p.y, z:p.z, w:0}, Cube::Inactive);
                }
                b'#' => {
                    pocket_dim_3d.insert(p.clone(), Cube::Active);
                    pocket_dim_4d.insert(Vec4{x:p.x, y:p.y, z:p.z, w:0}, Cube::Active);
                }
                _ => (),
            };
            p += Vec3{x:1, y:0, z:0};
        }
        p = Vec3{x:0, y:p.y + 1, z:0};
    }

    macro_rules! simulate {
        ($pocket_dim:ident, $dirs_fn:ident) => {
            // Expand out pocket_dim
            let mut next_pocket_dim = $pocket_dim.clone();
            for (pos, _cube) in &next_pocket_dim {
                for dir in $dirs_fn() {
                    $pocket_dim.entry(*pos + *dir).or_insert(Cube::Inactive);
                }
            }
            next_pocket_dim = $pocket_dim.clone();

            for (pos, cube) in &$pocket_dim {
                let mut active_neighbors = 0;
                for dir in $dirs_fn() {
                    if let Some(c) = $pocket_dim.get(&(*pos + *dir)) {
                        if *c == Cube::Active {
                            active_neighbors += 1;
                        }
                    }
                }
                if *cube == Cube::Active && !(active_neighbors == 2 || active_neighbors == 3) {
                    next_pocket_dim.insert(pos.clone(), Cube::Inactive);
                } else if *cube == Cube::Inactive && active_neighbors == 3 {
                    next_pocket_dim.insert(pos.clone(), Cube::Active);
                }
            }
            $pocket_dim = next_pocket_dim;
        };
    }

    for _i in 0..6 {
        simulate!(pocket_dim_3d, dirs_3d);
        simulate!(pocket_dim_4d, dirs_4d);
    }

    let part1 = pocket_dim_3d.iter().filter(|(_, cube)| **cube == Cube::Active).count();
    println!("Part 1: {}", part1);
    
    let part2 = pocket_dim_4d.iter().filter(|(_, cube)| **cube == Cube::Active).count();
    println!("Part 2: {}", part2);
}
