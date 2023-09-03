use std::{io, ops::Add};

fn main() {
    println!("****** Hypotenuse calculator ******");

    let mut side_a_buf = String::new();
    let mut side_b_buf = String::new();

    let side_a: f64;
    let side_b: f64;

    loop {
        println!("Enter side A:");

        io::stdin()
            .read_line(&mut side_a_buf)
            .expect("Could not read sideA");

        side_a = match side_a_buf.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Could not parse side A to number");
                continue;
            }
        };
        break;
    }

    loop {
        println!("Enter side B:");

        io::stdin()
            .read_line(&mut side_b_buf)
            .expect("Could not read sideB");

        side_b = match side_b_buf.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Could not parse side B to number");
                continue;
            }
        };
        break;
    }

    let hypotenuse = side_a.powi(2).add(side_b.powi(2)).sqrt();

    println!("Side C: {}", hypotenuse);
}
