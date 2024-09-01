fn main() {
    println!("Testing the nth fibonacci function");
    println!("0th fibonacci number is {}", nth_fibonacci(0)); // 0
    println!("1st fibonacci number is {}", nth_fibonacci(1)); // 1
    println!("2nd fibonacci number is {}", nth_fibonacci(2)); // 1
    println!("3rd fibonacci number is {}", nth_fibonacci(3)); // 2
    println!("4th fibonacci number is {}", nth_fibonacci(4)); // 3
    println!("5th fibonacci number is {}", nth_fibonacci(5)); // 5
    println!("10th fibonacci number is {}", nth_fibonacci(10)); // 55
}

fn nth_fibonacci(n: u32) -> u64 {
    // base case
    if n <= 1 {
        return n as u64;
    }
    // recursive case
    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
