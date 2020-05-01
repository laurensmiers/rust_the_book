use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let min_number = 1;
    let max_number = 100;
    let secret_number = rand::thread_rng().gen_range(min_number, max_number + 1);

    println!("Guess the number between [{}, {}] {}", min_number, max_number, secret_number);

    loop {
        let mut guess = String::new();
        println!("Input guess:");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => {
	        println!("That ain't a number!");
		continue;
	    }
	};

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congrats, you did it!");
                break;
            }
        }
    }

}
