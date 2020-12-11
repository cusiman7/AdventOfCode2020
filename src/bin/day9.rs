
use std::collections::VecDeque;
use std::collections::HashSet;
use std::cmp::{min, max};

fn is_valid_xmas_num(deque : &VecDeque<i64>, num : i64) -> bool {
    let mut nums = HashSet::new();
    for n in deque {
        let diff = num - n;
        if nums.contains(&diff) {
            return true;
        }
        nums.insert(n);
    }
    return false;
}

fn main() {
    let xmas : Vec<_> = aoc::file_lines_iter("./day9.txt")
        .map(|v| v.parse::<i64>().unwrap()).collect();

    let mut deque = VecDeque::new();
    let mut xmas_iter = xmas.iter();
    for _i in 0..25 {
        deque.push_back(*xmas_iter.next().unwrap());
    }

    let mut target : i64 = 0;
    for num in xmas_iter {
        if !is_valid_xmas_num(&deque, *num) {
            println!("Part 1: {}", num);
            target = *num;
            break;
        }
        deque.push_back(*num);
        deque.pop_front();
    }

    let mut sum : i64 = 0;
    let mut deque2 : VecDeque<i64> = VecDeque::new();
    for n in xmas {
        sum += n;
        deque2.push_back(n);
        while sum > target {
            sum -= deque2.pop_front().unwrap();
        }
        if sum == target {
            let (min, max) = deque2.iter().fold((i64::MAX, i64::MIN), |min_max, x| (min(*x, min_max.0), max(*x, min_max.1)));
            println!("Part 2: {}", min + max);
            break;
        }
    }
}
