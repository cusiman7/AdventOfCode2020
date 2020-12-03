
fn check_slope(ski_map : &Vec<Vec<char>>, dx : usize, dy : usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    while y < ski_map.len() {
        if ski_map[y][x] == '#' {
            tree_count += 1;
        }
        x = (x + dx) % (ski_map[0].len());
        y += dy;
    }
    return tree_count;
}

fn main() {
    let mut ski_map = Vec::new();
    if let Ok(lines) = aoc::read_lines("./day3.txt") {
        for line in lines {
            if let Ok(line) = line {
                ski_map.push(line.chars().collect::<Vec<_>>());
            }
        }
    }

    println!("Trees: {}", check_slope(&ski_map, 3, 1));

    let mut product : u64 = 1;
    for (dx, dy) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        product *= check_slope(&ski_map, *dx, *dy) as u64;
    }
    println!("Trees Product: {}", product);
}
