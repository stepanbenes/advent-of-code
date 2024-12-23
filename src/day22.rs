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

fn get_input() -> Vec<i64> {
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
