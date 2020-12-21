
use std::collections::HashMap;

fn main() {
    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut mem2 : HashMap<u64, u64> = HashMap::new();
    let mut mask : String = String::new();
    for line in aoc::file_lines_iter("./day14.txt") {
        let v : Vec<_> = line.split(" = ").collect();
        if v[0].starts_with("mask") {
            mask = v[1].to_string();
        } else {
            let lb = v[0].find("[").unwrap() + 1;
            let rb = v[0].find("]").unwrap();

            let addr = v[0].get(lb..rb).unwrap().parse::<u64>().unwrap();
            let mut value = v[1].parse::<u64>().unwrap();
            let value2 = value.clone();
            let mut x_count = 0;

            // Part 1
            for i in (0..36).rev() {
                match mask.as_bytes()[i] {
                    b'0' => value &= !(1 << (35 - i)),
                    b'1' => value |= 1 << (35 - i),
                    b'X' => x_count += 1,
                    _ => (),
                }
            }
            mem.insert(addr, value);

            // Part 2: enumerate all possible addresses
            for mut x in 0..2_u64.pow(x_count) {
                let mut final_addr = 0;
                for i in (0..36).rev() {
                    match mask.as_bytes()[i] {
                        b'0' => final_addr |= addr & (1 << (35 - i)),
                        b'1' => final_addr |= 1 << (35 - i),
                        b'X' => {
                            final_addr |= (x & 1) << (35 - i);
                            x >>= 1;
                        }
                        _ => (),
                    }
                }
                mem2.insert(final_addr, value2);
            }
        }
    }

    let part1 : u64 = mem.values().sum();
    println!("Part 1: {}", part1);

    let part2 : u64 = mem2.values().sum();
    println!("Part 2: {}", part2);
}

