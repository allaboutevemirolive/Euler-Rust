
fn multiples_of_3_and_5(number: i32) -> i32 {
    let mut sum = 0;
    let mut i = 3;

    while i < number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
    }

    sum
}

fn multiples_of_3_and_5(number: i32) -> i32 {
    // sum of multiples of 3
    let num_mult_of_3 = (number - 1) / 3;
    let sum_3 = (num_mult_of_3 * (num_mult_of_3 + 1) / 2) * 3;

    // sum of multiples of 5
    let num_mult_of_5 = (number - 1) / 5;
    let sum_5 = (num_mult_of_5 * (num_mult_of_5 + 1) / 2) * 5;

    // sum of multiples of 15 (both 3 and 5)
    // we need to subtract these because they are added twice
    let num_mult_of_15 = (number - 1) / 15;
    let sum_15 = (num_mult_of_15 * (num_mult_of_15 + 1) / 2) * 15;

    sum_3 + sum_5 - sum_15
}

fn main() {
    let number = 10;
    let result = multiples_of_3_and_5(number);
    println!("The sum of multiples of 3 or 5 below {} is {}", number, result);
}
