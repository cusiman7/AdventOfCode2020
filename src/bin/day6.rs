use std::collections::HashSet;

fn main() {
    let mut anyone_count = 0;
    let mut everyone_count = 0;
    let mut anyone = HashSet::new();
    let mut everyone = HashSet::new(); 
    let mut first_person = true;
    for line in aoc::read_lines_as_vec("./day6.txt") {
        if line.is_empty() {
            anyone_count += anyone.len();
            anyone.clear();

            everyone_count += everyone.len();
            everyone.clear();
            first_person = true;
            continue;
        }
        let person : HashSet<_> = line.chars().collect();
        anyone.extend(line.chars());
        if first_person == true {
            everyone = person;
            first_person = false;
        } else {
            everyone = everyone.intersection(&person).cloned().collect();
        }
    }
    anyone_count += anyone.len();
    everyone_count += everyone.len();
    println!("Total Answered Q's: {}", anyone_count);
    println!("Total Answered Q's 2: {}", everyone_count);
}
