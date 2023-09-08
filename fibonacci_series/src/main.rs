fn main() {
    println!("Hello, world!");

    let series = fibonacci_series(100);
    for i in series {
        println!("{i} ");
    }
}

fn fibonacci_series(num: u32) -> Vec<u128> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    let mut sum = a + b;
    let mut count = 1;

    let mut series = Vec::new();
    while count <= num {
        count += 1;

        series.push(a);

        a = b;
        b = sum;
        sum = a + b;
    }

    series
}