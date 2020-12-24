
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    All(Vec<i32>),
    Char(u8),
    Either(Vec<i32>, Vec<i32>),
    NFollowedByM(i32, i32),
}

struct Engine<'a> {
    rules: &'a HashMap<i32, Rule>,
}

impl Engine<'_> {
    fn match_message(&self, msg: &str) -> bool {
        let mut byte_index: usize = 0;
        self.match_rule(msg.as_bytes(), 0, &mut byte_index) && byte_index == msg.len()
    }

    fn match_rule(&self, msg: &[u8], rule_id: i32, byte_index: &mut usize) -> bool {
        if byte_index >= &mut msg.len() {
            return false;
        }
        match self.rules.get(&rule_id).unwrap() {
            Rule::All(sub_rules) => {
                let mut i = *byte_index;
                if sub_rules.iter().all(|r| self.match_rule(msg, *r, &mut i)) {
                    *byte_index = i;
                    true
                } else {
                    false
                }
            }
            Rule::Char(c) => {
                let i = *byte_index;
                *byte_index += 1;
                *c == msg[i]
            }
            Rule::Either(a, b) => {
                let mut i1 = *byte_index;
                let mut i2 = *byte_index;
                if a.iter().all(|r| self.match_rule(msg, *r, &mut i1)) {
                    *byte_index = i1;
                    true
                } else if b.iter().all(|r| self.match_rule(msg, *r, &mut i2)) {
                    *byte_index = i2;
                    true
                } else {
                    false
                }
            }
            Rule::NFollowedByM(a, b) => {
                // I'm too tired/dumb to parse this grammar fully, so I'm just going to handle the
                // rules I have
                // After looking at what rule 0: 8 11 evaluates to it's basically 42n42m31m
                // Basic form will look like 42, 42... 31...
                // At least 2 42s followed by a different, smaller, number of 31s
                let mut a_count = 0;
                let mut i = *byte_index;
                if !self.match_rule(msg, *a, &mut i) {
                    return false;
                }
                a_count += 1;
                // Thankfully a_step is constant across all branches
                let a_step = i - *byte_index;
                while self.match_rule(msg, *a, &mut i) {
                    a_count += 1;
                }
                while a_count > 1 {
                    let mut i2 = i;
                    let mut b_count = 0;
                    while self.match_rule(msg, *b, &mut i2) {
                        b_count += 1;
                    }
                    if b_count > 0 && b_count < a_count && msg.len() == i2 {
                        *byte_index = i2;
                        return true;
                    }
                    
                    i -= a_step;
                    a_count -= 1;
                }
                false
            }
        }
    }
}


fn main() {
    let mut lines = aoc::file_lines_iter("./day19.txt");
    let mut line = lines.next().unwrap();

    let mut rules : HashMap<i32, Rule> = HashMap::new();
    while !line.is_empty() {
        let v: Vec<_> = line.split(":").collect();
        let id = v[0].parse::<i32>().unwrap();
        if v[1].contains("|") {
            let mut sub_rules = v[1].split("|")
                .map(|sub| sub.trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>());
            rules.insert(id, Rule::Either(sub_rules.next().unwrap(), sub_rules.next().unwrap()));
        } else if v[1].contains("\"") {
            let c = v[1].trim().as_bytes()[1];
            rules.insert(id, Rule::Char(c));
        } else {
            let all: Vec<i32> = v[1].trim().split(" ").map(|i| i.parse::<i32>().unwrap()).collect();
            rules.insert(id, Rule::All(all));
        }
        line = lines.next().unwrap();
    }

    let messages: Vec<_> = lines.collect();

    let engine = Engine{rules: &rules};
    let part1 = messages.iter().filter(|m| engine.match_message(m)).count(); 
    println!("Part 1: {}", part1);

    let mut new_rules = rules.clone(); 
    new_rules.insert(8, Rule::Either(vec![42], vec![42, 8]));
    new_rules.insert(11, Rule::Either(vec![42, 31], vec![42, 11, 31]));
    // we're just gonna cheat
    new_rules.insert(0, Rule::NFollowedByM(42, 31));

    let engine2 = Engine{rules: &new_rules};
    let part2 = messages.iter().filter(|m| engine2.match_message(m)).count();
    println!("Part 2: {}", part2);
}
