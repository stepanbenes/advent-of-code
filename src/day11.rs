use std::collections::LinkedList;

pub fn count_of_stones() -> u32 {
    let mut stones = get_input();
    // TODO: use Linked list instead of Vec

    //println!("{:?}", stones);
    for i in 1..=25 {
        blink(&mut stones);
        //println!("{:?}", stones);
        println!("{i}: {:?}", stones.len());
    }
    stones.len() as u32
}

fn get_input() -> LinkedList<u64> {
    //let input = "0 1 10 99 999";
    //let input = "125 17";
    let input = "112 1110 163902 0 7656027 83039 9 74";
    let mut stones = LinkedList::new();
    for stone in input.split_whitespace() {
        stones.push_back(stone.parse::<u64>().unwrap());
    }
    stones
}

fn blink(stones: &mut LinkedList<u64>) {
    let mut cursor = stones.cursor_front_mut();
    while let Some(current) = cursor.current() {
        // 1. If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        if *current == 0 {
            *current = 1;
        }
        // 2. If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        else if current.to_string().len() % 2 == 0 {
            let mut digits = current.to_string();
            let half = digits.len() / 2;
            let left = digits.split_off(half);
            let right = digits;
            let left_value = left.parse::<u64>().unwrap();
            let right_value = right.parse::<u64>().unwrap();
            *current = left_value;
            cursor.insert_after(right_value);
            cursor.move_next();
        }
        // 3. If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
        else {
            *current *= 2024;
        }
        cursor.move_next();
    }
}
