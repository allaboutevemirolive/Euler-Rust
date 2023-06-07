fn fibo_even_sum(number: u32) -> u32 {
    if number <= 1 {
        return 0;
    } else {
        let mut even_sum = 0;
        let mut prev_fib_num = 1;
        let mut fib_num = 2;
        
        while fib_num <= number {
            if fib_num % 2 == 0 {
                even_sum += fib_num;
            }
            
            let temp = fib_num;
            fib_num += prev_fib_num;
            prev_fib_num = temp;
        }
        
        return even_sum;
    }
}

fn main() {
    let number = 34;
    let result = fibo_even_sum(number);
    println!("The sum of even Fibonacci numbers up to {} is {}", number, result);
}
