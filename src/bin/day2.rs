
fn main() {
    if let Ok(lines) = aoc::read_lines("./day2.txt") {
        let mut valid_count = 0;
        let mut valid_count2 = 0;
        for line in lines {
            if let Ok(line) = line {
                let v : Vec<_> =  line.split(": ").collect();
                let password = v[1];
                let v : Vec<_> = v[0].split(" ").collect();
                let letter : char = v[1].parse().unwrap();
                let v : Vec<_> = v[0].split("-").collect();
                let min : usize = v[0].parse().unwrap();
                let max : usize = v[1].parse().unwrap();

                let count = password.chars().filter(|&c| c == letter).count();
                if count >= min && count <= max {
                    valid_count += 1;
                }

                if (password.chars().nth(min-1).unwrap() == letter) ^
                   (password.chars().nth(max-1).unwrap() == letter) {
                    valid_count2 += 1;
                }
            }
        }
        println!("Part1: {}", valid_count);
        println!("Part2: {}", valid_count2);
    }
}
