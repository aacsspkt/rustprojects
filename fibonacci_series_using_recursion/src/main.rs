fn main() {
    // computation becomes slow after 40 iteration!
    let n = 40;

    println!("Fibonacci series:");
    for i in 1..=n {
        println!("F({}) {}",i, fibonacci_series(i));
    }
}

fn fibonacci_series(n: u64) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_series(n - 1) + fibonacci_series(n - 2)
    }
}
