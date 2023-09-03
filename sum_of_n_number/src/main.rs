fn main() {
    println!("Sum of 'n' Number Calculator!");

    let mut num_buf = String::new();
    let num: u32;

    loop {
        println!("Enter a number:");
        std::io::stdin()
        .read_line(&mut num_buf)
        .expect("Could not read data");

        num = match num_buf.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Could not parse to number");
                continue;
            }
        };
        break;
    }

    let sum = (num * (num + 1)) as f64 / 2_f64;

    println!("Sum of {} number is {}", num, sum);
}
