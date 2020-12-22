
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Rule(RangeInclusive<i64>, RangeInclusive<i64>);

fn main() {
    let mut lines = aoc::file_lines_iter("./day16.txt");
    let mut rules : HashMap<String, Rule> = HashMap::new();
    let mut line = lines.next().unwrap();

    while !line.is_empty() {
        let v : Vec<_> = line.split(": ").collect();
        let ranges : Vec<_> = v[1].split(" or ").collect();
        let range1 : Vec<_> = ranges[0].split("-").map(|v| v.parse::<i64>().unwrap()).collect();
        let range2 : Vec<_> = ranges[1].split("-").map(|v| v.parse::<i64>().unwrap()).collect();
        rules.insert(v[0].to_string(), Rule(range1[0]..=range1[1], range2[0]..=range2[1]));
        line = lines.next().unwrap();
    }

    line = lines.next().unwrap();
    assert!(line == "your ticket:");
    line = lines.next().unwrap();
    let my_ticket : Vec<i64> = aoc::csv_iter::<i64>(&line).collect();

    line = lines.nth(1).unwrap();
    assert!(line == "nearby tickets:");
    let mut invalid_sum = 0;
    let mut valid_tickets : Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let ticket : Vec<i64> = aoc::csv_iter::<i64>(&line).collect();
        let mut valid_ticket = true;
        for value in &ticket {
            let valid = rules.values().any(|Rule(r1, r2)| {
                r1.contains(&value) || r2.contains(&value)
            });
            if !valid {
                invalid_sum += value;
                valid_ticket = false;
            }
        }
        if valid_ticket {
            valid_tickets.push(ticket);
        }
    }
    println!("Part 1: {}", invalid_sum);

    valid_tickets.push(my_ticket.clone());
    let ticket_fields_count = valid_tickets[0].len();

    let mut rule_to_field_set : HashMap<String, HashSet<usize>> = HashMap::new();
    for (name, Rule(r1, r2)) in &rules {
        for i in 0..ticket_fields_count {
            if valid_tickets.iter().all(|t| r1.contains(&t[i]) || r2.contains(&t[i])) {
                rule_to_field_set.entry(name.clone()).or_insert(HashSet::new()).insert(i);
            }
        }
    }

    let mut rule_to_field : HashMap<String, usize> = HashMap::new();
    loop {
        // drain_filter would be nice, but it's in nightly right now
        // I think this is the first time I truly "fought" the borrow checker
        let mut rule : String = String::new();
        let mut pos : usize = 0;
        for (r, set) in &mut rule_to_field_set {
            if set.len() == 1 {
                rule = r.to_string();
                pos = set.drain().nth(0).unwrap();
                rule_to_field.insert(r.to_string(), pos);
                break;
            }
        }
        rule_to_field_set.remove(&rule);
        if rule_to_field_set.is_empty() {
            break;
        }
        for (_r, set) in &mut rule_to_field_set {
            set.remove(&pos);
        }
    }
    
    let part2 : i64 = rules.iter()
        .filter(|(k, _v)| k.starts_with("departure"))
        .map(|(k, _v)| my_ticket[*rule_to_field.get(k).unwrap()])
        .product();

    println!("Part 2: {}", part2);
}

