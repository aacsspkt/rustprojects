fn main() {
    println!("Sum of multiple of 3 and 5 in 'n' number Calculator!");

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

    let sum = calculate_sum(num);

    println!("The sum of multiple of 3 and 5 in {} number is {}", num, sum);
}

pub fn calculate_sum<'a>(num: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=num{
        if i % 3 == 0 || i % 5 == 0 {
            sum+=i;
        }
    }

    return sum;
}
