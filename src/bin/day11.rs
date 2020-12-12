
static DIRS : [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

fn permute_seats<F>(seats : &Vec<char>, w : usize, h: usize, limit : usize, f : F) -> Vec<char> 
where F : Fn(usize, usize) -> usize {
    let mut res = seats.clone();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if seats[i] == 'L' && f(x, y) == 0 {
                res[i] = '#';
            } else if seats[i] == '#' && f(x, y) >= limit {
                res[i] = 'L';
            }
        }
    }
    return res;
}

fn seating_rules(seats : &Vec<char>, w : usize, h : usize) -> Vec<char> {
    let adj_seats_occupied = |x : usize, y : usize| {
        DIRS.iter()
            .map(move |(dx, dy)| (x as i32 + dx, y as i32 + dy))
            .filter(|(new_x, new_y)| (0..w as i32).contains(new_x) && (0..h as i32).contains(new_y))
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| seats[y * w + x] == '#').count()
    };
    permute_seats(seats, w, h, 4, adj_seats_occupied)
}

fn seating_rules2(seats : &Vec<char>, w : usize, h : usize) -> Vec<char> {
    let vis_seats_occupied = |x : usize, y : usize| {
        let mut occupied = 0;
        for (dx, dy) in DIRS.iter() {
            let mut new_x = x as i32 + dx;
            let mut new_y = y as i32 + dy;
            loop {
                if !((0..w as i32).contains(&new_x) && (0..h as i32).contains(&new_y)) {
                    break;
                }
                let i = new_y * w as i32 + new_x;
                match seats[i as usize] {
                    'L' => break,
                    '#' => {
                        occupied += 1;
                        break;
                    }
                    _ => (),
                }
                new_x += dx;
                new_y += dy;
            }
        }
        return occupied;
    };
    permute_seats(seats, w, h, 5, vis_seats_occupied)
}

fn repeat_apply_rules<F>(init_seats : Vec<char>, w : usize, h : usize, f : F) -> usize
where F : Fn(&Vec<char>, usize, usize) -> Vec<char> {
    let mut seats = init_seats;
    loop {
        let new_seats = f(&seats, w, h);
        if new_seats == seats {
            return new_seats.iter().filter(|seat| **seat == '#').count();
        }
        seats = new_seats;
    }
}

fn main() {
    let mut w : usize = 0;
    let mut h : usize = 0;
    let mut init_seats : Vec<char> = Vec::new();

    let mut v : Vec<char> = Vec::new();
    for line in aoc::file_lines_iter("./day11.txt") {
        v.clear();
        v.extend(line.chars());
        w = v.len();
        h += 1;
        init_seats.append(&mut v); 
    }

    let part1 = repeat_apply_rules(init_seats.clone(), w, h, seating_rules);
    println!("Part 1: {}", part1);

    let part2 = repeat_apply_rules(init_seats.clone(), w, h, seating_rules2);
    println!("Part 2: {}", part2);
}
