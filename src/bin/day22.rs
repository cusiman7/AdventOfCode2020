
use std::collections::{VecDeque, HashSet};

fn recursive_combat(game: i64, mut p1_deck: VecDeque<i64>, mut p2_deck: VecDeque<i64>) -> bool {
    let mut p1_history: HashSet<VecDeque<i64>> = HashSet::new();
    let mut p2_history: HashSet<VecDeque<i64>> = HashSet::new();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        if p1_history.contains(&p1_deck) && p2_history.contains(&p2_deck) {
            return true;
        }
        p1_history.insert(p1_deck.clone());
        p2_history.insert(p2_deck.clone());

        let p1 = p1_deck.pop_front().unwrap() as usize;
        let p2 = p2_deck.pop_front().unwrap() as usize;
        let p1_wins;
        if p1 <= p1_deck.len() && p2 <= p2_deck.len() {
            p1_wins = recursive_combat(game + 1, p1_deck.clone().iter().cloned().take(p1).collect(), 
                                       p2_deck.clone().iter().cloned().take(p2).collect());
        } else {
            p1_wins = p1 > p2;
        }
        if p1_wins {
            p1_deck.push_back(p1 as i64);
            p1_deck.push_back(p2 as i64);
        } else {
            p2_deck.push_back(p2 as i64);
            p2_deck.push_back(p1 as i64);
        }
    }
    if game == 1 {
        if !p1_deck.is_empty() {
            let part2 = p1_deck.iter().rev().enumerate().fold(0_i64, |acc, (i, v)| acc + (i as i64 + 1) * v);
            println!("Part 2: {}", part2);
        } else {
            let part2 = p2_deck.iter().rev().enumerate().fold(0_i64, |acc, (i, v)| acc + (i as i64 + 1) * v);
            println!("Part 2: {}", part2);
        }
    }
    if !p1_deck.is_empty() {
        return true;
    }
    return false;
}

fn main() {
    let mut p1_deck: VecDeque<i64> = VecDeque::new();
    let mut p2_deck: VecDeque<i64> = VecDeque::new();

    let mut parsing_p1 = false;
    let mut parsing_p2 = false;
    for line in aoc::file_lines_iter("./day22.txt") {
        if line == "Player 1:" {
            parsing_p1 = true;
            parsing_p2 = false;
            continue;
        } else if line == "Player 2:" {
            parsing_p1 = false;
            parsing_p2 = true;
        } else if line.is_empty() {
            continue;
        } else if parsing_p1 {
            p1_deck.push_back(line.parse::<i64>().unwrap());
        } else if parsing_p2 {
            p2_deck.push_back(line.parse::<i64>().unwrap());
        }
    }

    let p1_deck2 = p1_deck.clone();
    let p2_deck2 = p2_deck.clone();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1 = p1_deck.pop_front().unwrap();
        let p2 = p2_deck.pop_front().unwrap();

        if p1 > p2 {
            p1_deck.push_back(p1);
            p1_deck.push_back(p2);
        } else {
            p2_deck.push_back(p2);
            p2_deck.push_back(p1);
        }
    }

    if !p1_deck.is_empty() {
        let score = p1_deck.iter().rev().enumerate().fold(0_i64, |acc, (i, v)| acc + (i as i64 + 1) * v);
        println!("Part 1: {}", score);
    } else {
        let score = p2_deck.iter().rev().enumerate().fold(0_i64, |acc, (i, v)| acc + (i as i64 + 1) * v);
        println!("Part 1: {}", score);
    }
    
    recursive_combat(1, p1_deck2.clone(), p2_deck2.clone());
}
