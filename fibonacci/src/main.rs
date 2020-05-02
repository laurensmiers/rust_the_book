use std::io;

const FIB_0: u32 = 0;
const FIB_1: u32 = 1;

fn generate_fib_number(n: u32) -> u32 {
    if n == 0 {
        FIB_0
    } else if n == 1 {
        FIB_1
    } else {
        generate_fib_number(n - 1) + generate_fib_number(n - 2)
    }
}

fn print_fib_sequence(n: u32) {
    println!("Just for lullz, a sequence of {} fibonacci numbers", n);
    for n in 0..n {
        println!("{}: {}", n, generate_fib_number(n));
    }
}

fn main() {
    let mut n = String::new();

    println!("Input number of fibonacci numbers to generate:");
    io::stdin().read_line(&mut n).expect("Failed to read user input");

    let n: u32 = n.trim().parse().expect("Not a number!");

    print_fib_sequence(n);
}
