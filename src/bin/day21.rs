
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::cell::RefCell;

fn main() {
    let mut i_to_a: HashMap<String, HashSet<String>> = HashMap::new();
    let mut a_to_i: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_recipes: Vec<(HashSet<String>, HashSet<String>)> = Vec::new();
    let mut all_ingredients: HashSet<String> = HashSet::new();
    let mut all_allergens: HashSet<String> = HashSet::new();

    for line in aoc::file_lines_iter("./day21.txt") {
        let contains = " (contains ";
        let end_of_ingredients = line.find(contains).unwrap();
        let end_of_allergens = line.rfind(")").unwrap();

        let ingredients: Vec<String> = line[0..end_of_ingredients].split(" ").map(|s| s.to_string()).collect();
        let allergens: Vec<String> = line[(end_of_ingredients + contains.len())..end_of_allergens].split(", ").map(|s| s.to_string()).collect(); 

        for i in &ingredients {
            all_ingredients.insert(i.clone());
            i_to_a.entry(i.clone()).or_insert(HashSet::new()).extend(allergens.clone());
        }

        for a in &allergens {
            all_allergens.insert(a.clone());
            a_to_i.entry(a.clone()).or_insert(HashSet::new()).extend(ingredients.clone());
        }

        all_recipes.push((HashSet::from_iter(ingredients), HashSet::from_iter(allergens)));
    }

    let mut safe_ingredients: HashSet<String> = HashSet::new();
    for i in &all_ingredients {
        let mut set: HashSet<String> = i_to_a.get(i).unwrap().clone();
        for (ingredients, allergens) in all_recipes.iter() {
            if set.intersection(allergens).count() > 0 {
                if !ingredients.contains(i) {
                    for a in allergens {
                        set.remove(a);
                    }
                }
            }
        }
        if set.is_empty() {
            safe_ingredients.insert(i.clone());
        }
    }

    let mut part1 = 0;
    for (i, _a) in &all_recipes {
        for safe_i in &safe_ingredients {
            if i.contains(safe_i) {
                part1 += 1;
            }
        }
    }
    println!("Part 1: {}", part1);

    let mut bad_a_to_i: HashMap<String, RefCell<HashSet<String>>> = HashMap::new();
    for aller in &all_allergens {
        let mut ingredients = a_to_i.get(aller).unwrap().clone();
        for (i, _a) in all_recipes.iter().filter(|(_i, a)| a.contains(aller)) {
            ingredients = ingredients.intersection(&i).map(|s| s.to_string()).collect();
        }
        bad_a_to_i.insert(aller.clone(), RefCell::new(ingredients));
    }

    loop {
        for (a, i) in &bad_a_to_i {
            if i.borrow().len() == 1 {
                for (a2, i2) in &bad_a_to_i {
                    if a == a2 {
                        continue;
                    }
                    i2.borrow_mut().remove(i.borrow().iter().next().unwrap());
                }
            }
        }
        if bad_a_to_i.iter().all(|(_a, i)| i.borrow().len() == 1) {
            break;
        }
    }

    let mut allergens_alphabetically: Vec<String> = all_allergens.iter().map(|s| s.to_string()).collect();
    allergens_alphabetically.sort();

    print!("Part 2: ");
    let mut iter = allergens_alphabetically.iter().peekable();
    while let Some(a) = iter.next() {
        print!("{}", bad_a_to_i.get(a).unwrap().borrow().iter().next().unwrap());
        if iter.peek() != None {
            print!(",");
        }
    }
    println!("");
}

