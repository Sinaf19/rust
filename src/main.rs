
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    // guessing_game();
    another_function(45);

}

fn guessing_game() {

    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim()
            .parse() {
            Ok(num) => num,
            // _ catchcall value
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
        }
    }
}

fn another_function(x: i32) -> String {
    println!("The value of x is: {x}");
    "Hello".to_string()
}