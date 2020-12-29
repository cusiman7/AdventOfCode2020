
fn main() {
    let subject = 7_i64;
    let door_pub_key = 10604480;
    let card_pub_key = 4126658;
    let mut value = 1_i64;

    let mut door_count = 0;
    loop {
        value *= subject;
        value = value % 20201227;
        door_count += 1;
        if value == door_pub_key {
            break;
        }
    }

    let mut card_count = 0;
    value = 1_i64;
    loop {
        value *= subject;
        value = value % 20201227;
        card_count += 1;
        if value == card_pub_key {
            break;
        }
    }

    let mut card_key = 1_i64;
    for _ in 0..door_count {
        card_key *= card_pub_key;
        card_key = card_key % 20201227;
    }

    let mut door_key = 1_i64;
    for _ in 0..card_count {
        door_key *= door_pub_key;
        door_key = door_key % 20201227; 
    }
    assert!(door_key == card_key);
    println!("Part 1: {}", card_key);
}

