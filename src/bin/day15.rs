
use std::collections::HashMap;

fn main() {
    let mut turn = 0;  
    // number -> (Most recent turn spoken, Turn before that)
    let mut hist : HashMap<i32, (i32, Option<i32>)> = HashMap::new();
    let mut last_num = 0;

    // Starting turns
    for line in aoc::file_lines_iter("./day15.txt") {
        let v : Vec<_> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        for num in v {
            hist.insert(num, (turn, None));
            last_num = num;
            turn += 1;
        }
    }

    loop {
        if turn == 2020 {
            println!("Part 1: {}", last_num);
        }
        if turn == 30000000 {
            println!("Part 2: {}", last_num);
            break;
        }
        let num_hist = hist.get(&last_num);
        last_num = num_hist.map_or(0, |(recent_turn, turn_before_that)| {
            if let Some(x) = turn_before_that {
                return recent_turn - x;
            } else {
                return 0;
            }
        });
        // "Speak" the num
        if let Some(last_num_hist) = hist.get_mut(&last_num) {
            *last_num_hist = (turn, Some(last_num_hist.0));
        } else {
            hist.insert(last_num, (turn, None));
        }
        turn += 1;
    }
}
