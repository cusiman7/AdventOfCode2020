
use std::collections::VecDeque;
use std::iter::FromIterator;

fn crab_cups(cups: &mut VecDeque<i32>, rounds: i32) {
    let min_label: i32 = *cups.iter().min().unwrap();
    let max_label: i32 = *cups.iter().max().unwrap();

    let next_destination_label = |destination_label: i32| -> i32 {
        let mut next = destination_label - 1;
        if next < min_label {
            next = max_label;
        }
        next
    };

    let mut picked_up: Vec<i32> = Vec::with_capacity(3);
    
    for i in 0..rounds {
        for _ in 0..3 {
            picked_up.push(cups.remove(1).unwrap());
        }

        let mut destination_label = cups[0];
        destination_label = next_destination_label(destination_label); 
        while picked_up.iter().find(|&&cup| cup == destination_label).is_some() {
            destination_label = next_destination_label(destination_label); 
        }
        let destination_pos = cups.iter().position(|&cup| cup == destination_label).unwrap();
        for (i, cup) in picked_up.iter().enumerate() {
            cups.insert(destination_pos + i + 1, *cup);
        }
        picked_up.clear();
        cups.rotate_left(1);

        if i % 10_000 == 0 {
            println!("{}", i);
        }
    }
    cups.rotate_left(cups.iter().position(|&cup| cup == 1).unwrap());
}

// for the record, it took me less time to code this than it did to run crab_cups()...
fn crab_cups_2000() {
    // cups don't move, what's clockwise to them changes though...
    // index == cup's label, value == next cup clockwise
    let mut cups: Vec<i32> = Vec::with_capacity(1_000_001);
    cups.resize(1_000_001, 0); 
    cups[6] = 1;
    cups[1] = 4;
    cups[4] = 7;
    cups[7] = 5;
    cups[5] = 2;
    cups[2] = 8;
    cups[8] = 3;
    cups[3] = 9;
    cups[9] = 6;

    let min_label = 1_i32;
    let max_label = 1_000_000_i32;

    for i in 9..1_000_000 {
        cups[i as usize] = i + 1;
    }
    cups[1_000_000] = 6;

    let next_destination_label = |destination_label: i32| -> i32 {
        let mut next = destination_label - 1;
        if next < min_label {
            next = max_label;
        }
        next
    };

    let mut current = 6_i32;
    for _ in 0..10_000_000 {
        // "pick up" 3 cups
        let cup1 = cups[current as usize]; 
        let cup2 = cups[cup1 as usize];
        let cup3 = cups[cup2 as usize];

        // Whatever was clockwise of cup3 is now clockwise of current 
        cups[current as usize] = cups[cup3 as usize];

        let mut destination = next_destination_label(current);
        while destination == cup1 || destination == cup2 || destination == cup3 {
            destination = next_destination_label(destination);
        }
        // "Place" the cups back down, cup1 already points to cup2 and cup2 already points to cup3
        let temp = cups[destination as usize];
        cups[destination as usize] = cup1;
        cups[cup3 as usize] = temp;

        current = cups[current as usize];
    }

    let cup1 = cups[1];
    let cup2 = cups[cup1 as usize];
    println!("Part 2: {}", cup1 as i64 * cup2 as i64);
}

fn main() {
    let mut cups = VecDeque::from_iter(vec![6, 1, 4, 7, 5, 2, 8, 3, 9].iter().cloned());
    crab_cups(&mut cups, 100);

    print!("Part 1: ");
    for i in 1..cups.len() {
        print!("{}", cups[i]);
    }
    println!("");

    crab_cups_2000();
}

