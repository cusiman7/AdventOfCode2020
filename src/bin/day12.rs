
use num::abs; 
use aoc::Vec2;

enum Dir {
    North,
    East,
    South,
    West,
}

fn main() {
    // Part 1
    let mut dir = Dir::East;
    let mut pos = Vec2 {x: 0, y: 0};

    // Part 2
    let mut way_pos = Vec2 {x: 10, y: 1};
    let mut pos2 = Vec2 {x: 0, y: 0};

    for line in aoc::file_lines_iter("./day12.txt") {
        let mut chars = line.chars();
        let action : char = chars.next().unwrap();
        let mut value : i32 = chars.collect::<String>().parse::<i32>().unwrap();

        match action {
            'N' => {
                pos += Vec2 {x: 0, y: value};
                way_pos += Vec2 {x: 0, y: value};
            }
            'S' => {
                pos += Vec2 {x: 0, y: -value};
                way_pos += Vec2 {x: 0, y: -value};
            }
            'E' => {
                pos += Vec2 {x: value, y: 0};
                way_pos += Vec2 {x: value, y: 0};
            }
            'W' => {
                pos += Vec2 {x: -value, y: 0};
                way_pos += Vec2 {x: -value, y: 0};
            }
            'L' => {
                while value != 0 {
                    dir = match dir {
                        Dir::North => Dir::West,
                        Dir::West => Dir::South,
                        Dir::South => Dir::East,
                        Dir::East => Dir::North,
                    };
                    way_pos = Vec2 {x: -way_pos.y, y: way_pos.x};
                    value -= 90;
                }
            }
            'R' => {
                while value != 0 {
                    dir = match dir {
                        Dir::North => Dir::East,
                        Dir::East => Dir::South,
                        Dir::South => Dir::West,
                        Dir::West => Dir::North,
                    };
                    way_pos = Vec2 {x: way_pos.y, y: -way_pos.x};
                    value -= 90;
                }
            }
            'F' => {
                match dir {
                    Dir::North => pos += Vec2 {x: 0, y: value},
                    Dir::South => pos += Vec2 {x: 0, y: -value},
                    Dir::East => pos += Vec2 {x: value, y: 0},
                    Dir::West => pos += Vec2 {x: -value, y: 0},
                }
                pos2 += way_pos * value;
            }
            _ => (),
        }
    }
    
    println!("Part1: {}", abs(pos.x) + abs(pos.y));
    println!("Part2: {}", abs(pos2.x) + abs(pos2.y));
}

