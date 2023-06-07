// Prime factor refers to a prime number that divides the given number evenly, without leaving any remainder.

// code 1
fn largest_prime_factor(number: u64) -> u64 {
    let mut largest_factor = number;

    for i in 2..=(largest_factor as f64).sqrt() as u64 {
        if largest_factor % i == 0 {
            let factor = largest_factor / i;
            let candidate = largest_prime_factor(factor);

            return if i > candidate { i } else { candidate };
        }
    }

    largest_factor
}

// code 2
fn largest_prime_factor(number: u64) -> u64 {
    let mut prime = 2;
    let mut max = 1;

    let mut n = number;
    while prime <= n {
        if n % prime == 0 {
            max = prime;
            n /= prime;
        } else {
            prime += 1;
        }
    }

    max
}

fn main() {
    let number: u64 = 13195;
    let result = largest_prime_factor(number);
    println!("The largest prime factor of {} is {}", number, result);
}
