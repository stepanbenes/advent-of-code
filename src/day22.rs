use std::collections::HashMap;

use itertools::Itertools;

pub fn sum_of_2000th_secret_numbers() -> i64 {
    let input = get_input();
    let mut sum = 0;
    for number in input {
        println!("Number: {}", number);
        let mut secret_number = number;
        for _ in 0..2000 {
            secret_number = get_next_secret_number(secret_number);
        }
        println!(" -> {}", secret_number);
        sum += secret_number;
    }
    sum
}

pub fn most_bananas_you_can_get() -> i64 {
    let input = get_input();
    let mut map: HashMap<(i8, i8, i8, i8), HashMap<usize, i8>> = HashMap::new();
    for (buyers_index, buyers_number) in input.iter().enumerate() {
        let mut secret_number = *buyers_number;
        for ((a, _), (b, _), (c, _), (d, price)) in (0..2000)
            .map(|_| {
                let old_value = secret_number;
                secret_number = get_next_secret_number(secret_number);
                old_value
            })
            .map(|x| (x % 10) as i8)
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| (b - a, b))
            .tuple_windows::<(_, _, _, _)>()
        {
            //println!("{:?}: {:?}", (a, b, c, d), price);
            let entry = map.entry((a, b, c, d)).or_default();
            entry.entry(buyers_index).or_insert(price);
        }
    }

    let max_item = map
        .iter()
        .map(|(key, value)| (key, value.values().map(|&p| p as i64).sum::<i64>()))
        .max_by_key(|(_, value)| *value)
        .unwrap();

    println!("{:?}", max_item);
    max_item.1
}

fn get_input() -> Vec<i64> {
    //let input = "123";
    //     let input = r"1
    // 2
    // 3
    // 2024";
    let input = include_str!("../input/day22.txt");
    //     let input = r"1
    // 10
    // 100
    // 2024";
    //let input = r"123";
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[allow(clippy::let_and_return)]
fn get_next_secret_number(number: i64) -> i64 {
    // Each secret number evolves into the next secret number in the sequence via the following process:

    // - Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    // - Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    // - Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.

    // Each step of the above process involves mixing and pruning:

    // - To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
    // - To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)

    fn mix(a: i64, b: i64) -> i64 {
        a ^ b
    }

    fn prune(a: i64) -> i64 {
        a % 16777216
    }

    let step1 = prune(mix(number * 64, number));
    let step2 = prune(mix(step1 / 32, step1));
    let step3 = prune(mix(step2 * 2048, step2));
    step3
}
