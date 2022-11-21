
use std::cmp::Ordering;
use std::io;
use rand::Rng;
use std::str::FromStr;

fn main() {

    // guessing_game();
    // another_function(45);
    // break_continue_loop();
    // for_loop();
    convert_f_to_c();

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



fn break_continue_loop() {

    /*
    un break ou un continue peut sortir
    d'un loop spécifique si label existant pour
    ce loop
     */


    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}



fn for_loop() {

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}

fn convert_f_to_c() {

    println!("Converter F -> C and C -> F\n");

    println!("Convert C to F ? (C = Celsius, F = Fahrenheit)");


    let mut unit = String::new();
    let celsius: String = String::from("Celsius");
    let fahrenheit: String = String::from("Fahrenheit");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit = unit.trim();

    println!("What is the value to convert ?");

    let mut to_convert = String::new();

    io::stdin()
        .read_line(&mut to_convert)
        .expect("Failed to read line");

    let to_convert: i32 = to_convert
        .trim()
        .parse()
        .expect("You have not entered the right value");


    if unit.eq(&celsius) {
        let mut value_in_F: f64 = 0.;
        value_in_F = (to_convert as f64 * (9. / 5.)) + 32.;
        println!("{} degrees Celsius is equal to {:.2} degrees Fahrenheit", to_convert, value_in_F);
    }

    if unit.eq(&fahrenheit) {
        let mut value_in_C: f64 = 0.;
        value_in_C = (to_convert as f64 - 32.) * (5. / 9.);
        println!("{} degrees Fahrenheit is equal to {:.2} degrees Celsius", to_convert, value_in_C);
    }

}




