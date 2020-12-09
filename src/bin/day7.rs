use std::collections::HashMap;


fn can_hold_shiny_gold(bag_map : &HashMap<String, Vec<(u64, String)>>, bag : &str) -> bool {
    for (_count, bag_type) in &bag_map[bag] {
        if bag_type == "shiny gold" {
            return true;
        }
        if can_hold_shiny_gold(bag_map, &bag_type) {
            return true;
        }
    }
    return false;
}

fn how_many_bags_must_a_bag_hold(bag_map : &HashMap<String, Vec<(u64, String)>>, bag :&str) -> u64 {
    return bag_map[bag].iter()
        .map(|(count, bag_type)| { 
            count * how_many_bags_must_a_bag_hold(bag_map, &bag_type)
        })
        .sum::<u64>() + 1; // +1 for itself
}


fn main() {

    let mut bag_map = HashMap::new();

    for line in aoc::read_lines_as_vec("./day7.txt") {
        let mut v = line.split(" bags contain ");
        let color = v.next().unwrap().to_string();
        let contents : Vec<_> = v.next().unwrap().split(',')
                                    .map(|content| content.trim_matches(|c| c == ' ' || c == '.'))
                                    .map(|content| content.splitn(2, ' ')) // Split out the # from the type of bag
                                    // Parse the # and type into a tuple (count, type)
                                    .map(|mut split_result| (split_result.next().unwrap().parse::<u64>().unwrap_or(0),
                                                             split_result.next().unwrap().split(" bag").nth(0).unwrap().to_string()))
                                    .filter(|(count, _)| count > &0)
                                    .collect();

        bag_map.insert(color, contents);
    }
    let total1 = bag_map.iter().filter(|(bag_type, _contents)| can_hold_shiny_gold(&bag_map, &bag_type)).count();
    println!("Total1: {}", total1);

    let total2 = how_many_bags_must_a_bag_hold(&bag_map, "shiny gold");
    println!("Total2: {}", total2 - 1); // -1 to not include the shiny gold bag itself
}

