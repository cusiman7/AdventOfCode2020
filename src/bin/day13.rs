
fn main() {

    let mut lines = aoc::file_lines_iter("./day13.txt");
    let departure = lines.next().unwrap().parse::<u64>().unwrap();
    let busses : Vec<_> = lines.next().unwrap().split(',').map(|s| s.to_string()).collect();
    let bus_ids : Vec <u64> = busses.iter().filter(|v| v.as_str() != "x")
                                  .map(|v| v.parse::<u64>().unwrap())
                                  .collect();

    let mut min_time = u64::MAX;
    let mut bus_id = 0;
    for id in bus_ids {
        let time = id - (departure % id);
        if time < min_time {
            min_time = time;
            bus_id = id;
        }
    }
    println!("Part 1: {}", bus_id * min_time);

    let mut bus_ids2 : Vec<(u64, u64)> = Vec::new();
    let mut offset = 0;
    for v in busses {
        if v.as_str() != "x" {
            bus_ids2.push((v.parse::<u64>().unwrap(), offset));
        }
        offset += 1;
    }
    
    let mut busses = bus_ids2.iter();
    let (mut i, mut step) = (0, busses.next().unwrap().0);
    for (id, offset) in busses {
        while (i + offset) % id != 0 {
            i += step;
        }
        step *= id;
    }
    println!("Part 2: {} ", i);
}

