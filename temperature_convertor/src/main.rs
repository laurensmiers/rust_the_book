use std::io;
use std::process::exit;

fn main() {
    let mut fahr = String::new();

    println!("Input fahrenheit number:");
    io::stdin().read_line(&mut fahr).expect("Failed to read user input");

    // let fahr: f64 = fahr.trim().parse().expect("Not a number!");
    let fahr: f64 = match fahr.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Can't work with that!");
            exit(1);
        }
    };


    let celsius = (fahr - 32.0) / 1.8;

    println!("{} Fahrenheit is {} in Degrees Celsius", fahr, celsius);
}
