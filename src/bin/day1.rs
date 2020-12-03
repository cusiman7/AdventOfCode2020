
use std::collections::HashSet;

fn part1(num_list: &Vec<i32>) {
    let mut nums = HashSet::new();
    for n in num_list {
        let diff = 2020 - n;
        if nums.contains(&diff) {
            let product = n * diff;
            println!("Product: {}", product);
            break;
        }
        nums.insert(n);
    }
}

fn part2(num_list: &Vec<i32>) {
    for (i, a) in num_list.iter().enumerate() {
        let mut nums = HashSet::new();
        for b in num_list.iter().skip(i) { 
            let diff = 2020 - a - b;
            if nums.contains(&diff) {
                let product = a * b * diff;
                println!("Product2: {}", product);
                return;
            }
            nums.insert(b);
        }
    }
}

fn main() {
    let mut vec = Vec::new();
    if let Ok(lines) = aoc::read_lines("./day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                vec.push(num.parse::<i32>().unwrap());
            }
        }
    }
    part1(&vec);
    part2(&vec);
}
