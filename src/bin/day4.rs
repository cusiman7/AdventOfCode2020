
use std::collections::HashMap;
use std::collections::HashSet;

fn has_required_fields(passport : &HashMap<String, String>) -> bool {
    let expected_fields : HashSet<&str> = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
    return expected_fields.iter().all(|&k| passport.contains_key(k));
}

fn fields_valid(passport : &HashMap<String, String>) -> bool {
    if !has_required_fields(passport) {
        return false;
    }

    if !passport["byr"].parse::<u32>().map_or(false, |byr| byr >= 1920 && byr <= 2002) {
        return false;
    }
    if !passport["iyr"].parse::<u32>().map_or(false, |iyr| iyr >= 2010 && iyr <= 2020) {
        return false;
    }
    if !passport["eyr"].parse::<u32>().map_or(false, |iyr| iyr >= 2020 && iyr <= 2030) {
        return false;
    }
    let good_cm : bool = passport["hgt"].find("cm")
                                 .and_then(|i| passport["hgt"].get(0..i))
                                 .and_then(|hgt| hgt.parse::<u32>().ok())
                                 .map_or(false, |hgt| hgt >= 150 && hgt <= 193);
    let good_in : bool = passport["hgt"].find("in")
                                 .and_then(|i| passport["hgt"].get(0..i))
                                 .and_then(|hgt| hgt.parse::<u32>().ok())
                                 .map_or(false, |hgt| hgt >= 59 && hgt <= 76);
    if !(good_cm || good_in) {
        return false;
    }
    if !passport["hcl"].len() == 7 || 
        passport["hcl"].chars().nth(0) != Some('#') ||
        !passport["hcl"].chars().skip(1).all(|c| match c {
            '0'..='9' | 'a'..='f' => true,
            _ => false
        }) {
        return false;
    }
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"].as_str()) {
        return false;
    }
    if !(passport["pid"].len() == 9 && passport["pid"].chars().all(char::is_numeric)) {
        return false;
    }
    
    return true;
}

fn main() {
    let mut fields : HashMap<String, String> = HashMap::new();
    let mut valid_count : i32 = 0;
    let mut valid_count2 : i32 = 0;
    if let Ok(lines) = aoc::read_lines("./day4.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    // Validate Passport
                    valid_count += has_required_fields(&fields) as i32;
                    valid_count2 += fields_valid(&fields) as i32;
                    fields.clear();
                }
                for field in line.split_whitespace() {
                    let v : Vec<&str> = field.split(':').collect();
                    fields.insert(v[0].to_string(), v[1].to_string());        
                }
            }
        }
    }
    if !fields.is_empty() {
        valid_count += has_required_fields(&fields) as i32;
        valid_count2 += fields_valid(&fields) as i32;
    }
    
    println!("Valid Passports: {}", valid_count);
    println!("Valid Passports 2: {}", valid_count2);
}
