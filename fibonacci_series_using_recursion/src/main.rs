use std::collections::HashMap;

fn main() {
    // computation becomes slow after 40 iteration!
    let n = 100;
    let mut memo:HashMap<u128, u128> = HashMap::new();
    
    println!("Fibonacci series:");
    for i in 1..=n {
        println!("F({}) {}", i, fibonacci_series_memoized(i, &mut memo));
    }
}

fn fibonacci_series_memoized(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if n <= 1 {
        return n;
    }

    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = fibonacci_series_memoized(n - 1, memo) + fibonacci_series_memoized(n - 2, memo);

    memo.insert(n, result);

    result
}
