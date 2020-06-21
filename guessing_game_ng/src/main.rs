use std::io;
use std::cmp::Ordering;
use rand::Rng;

const MIN_NUMBER: i32 = 1;
const MAX_NUMBER: i32 = 100;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < MIN_NUMBER || value > MAX_NUMBER {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(MIN_NUMBER, MAX_NUMBER + 1);

    println!("Guess the number between [{}, {}] {}", MIN_NUMBER, MAX_NUMBER, secret_number);

    loop {
        let mut guess = String::new();
        println!("Input guess:");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => {
	        println!("That ain't a number!");
		continue;
	    }
	};

        let guess: Guess = Guess::new(guess);

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congrats, you did it!");
                break;
            }
        }
    }

}
