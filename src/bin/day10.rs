
fn branches(i : usize, adapters : &Vec<i64>, memos :&mut Vec<u64>) -> u64 {
    if i == adapters.len() {
        return 0;
    }
    if memos[i] != 0 {
        return memos[i];
    }
    let mut res = branches(i+1, adapters, memos);
    if i+2 < adapters.len() && adapters[i+2] - adapters[i] <= 3 {
        res += 1 + branches(i+2, adapters, memos); 
    }
    if i+3 < adapters.len() && adapters[i+3] - adapters[i] <= 3 {
        res += 1 + branches(i+3, adapters, memos); 
    }
    memos[i] = res;
    return res;
}

fn main() {
    let mut adapters : Vec<_> = aoc::file_lines_iter("./day10.txt")
        .map(|v| v.parse::<i64>().unwrap()).collect();
    adapters.push(0); // The Seat
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3); // The Device

    let pairwise_adapters = adapters.iter().zip(adapters.iter().skip(1));
    let (ones, threes) = pairwise_adapters.fold((0, 0), |ones_threes, (a, b)| {
        match b - a {
            1 => (ones_threes.0 + 1, ones_threes.1),
            3 => (ones_threes.0, ones_threes.1 + 1),
            _ => ones_threes
        }
    });
    println!("Part 1: {}", ones * threes);

    let mut memos : Vec<u64> = Vec::new();
    memos.resize(adapters.len(), 0);
    let total_branches = 1 + branches(0, &adapters, &mut memos);
    println!("Part 2: {}", total_branches);
}
