fn main() {
    println!("Hello, world!");

    fibonacci_series(100);
}

fn fibonacci_series(num: u32) {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    let mut sum = a + b;
    let mut count = 1;

    while count <= num {
        count += 1;

        println!("{a}");

        a = b;
        b = sum;
        sum = a + b;
    }
}