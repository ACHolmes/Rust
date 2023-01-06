use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        if guess < secret {
            println!("Too small!");
        } else if guess > secret {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
