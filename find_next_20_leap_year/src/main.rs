fn main() {
    // Write a program that prints the next 20 leap years.

    // let mut year = 2023;
    let mut buf = String::new();
    let mut year: u32;
    loop {
        println!("Enter a year:");

        std::io::stdin()
        .read_line(&mut buf)
        .expect("Could not read data");

        year = match buf.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Could not parse number");
                continue;
            }
        };
        break;
    }

    let mut leap_year_found = 0;

    println!("Next 20 leap years:");
    while leap_year_found < 20 {
        if is_leap_year(year) {
            println!("{}", year);
            leap_year_found += 1;
        }
        year += 1;
    }
}


fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}