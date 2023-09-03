fn main() {
    println!("Multiplcation Table Printer");

    let mut num_buf = String::new();
    let num: u32;

    loop {
        println!("Enter a number to print to multiplication number:");

        std::io::stdin()
        .read_line(&mut num_buf)
        .expect("Could not read data");

        num = match num_buf.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Could not parse number!");
                continue;
            }
        };
        break;
    }

    for i in 1..=12 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
