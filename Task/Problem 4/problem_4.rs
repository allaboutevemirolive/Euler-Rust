fn largest_palindrome_product(n: u32) -> u32 {
    // To get the maximum n digit number
    let max: u32 = 10_u32.pow(n) - 1;

    // Next we get the minimum n digit number from the max
    let min: u32 = (max + 1) / 10;

    // To store the result
    let mut largest: u32 = 0;

    // Starting the loop from max to min
    for i in (min..=max).rev() {
        // Another loop
        for j in (min..=max).rev() {
            // Getting the product
            let num: u32 = i * j;
 
            // Reversing the number
            let num_reverse: u32 = num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap();

            // Checking for palindromic number
            if num == num_reverse {
                // Check if it's the largest and break the loop
                largest = largest.max(num);
                break;
            }
        }
    }

    // Returning the largest found
    largest
}

fn main() {
    let n: u32 = 2;
    let result = largest_palindrome_product(n);
    println!("The largest palindrome made from the product of two {}-digit numbers is {}", n, result);
}
