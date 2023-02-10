use crate::library::matrix::Matrix;

pub mod library;

fn main() {

    // guessing_game();
    // another_function(45);
    // break_continue_loop();
    // for_loop();
    // convert_f_to_c();

let test;
    let mut test1 = Matrix {rows: 3, cols: 3, data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]]};
    let test2 = Matrix {rows: 3, cols: 3, data: vec![vec![2.2, 2.3, 2.4], vec![2.5, 2.6, 2.7], vec![2.8, 2.9, 2.0]]};

    test = test1.multiply(&test2);

    println!("{:?}", test.data);

}


/*fn guessing_game() {

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

    println!("Which unit do you want to convert ? (C, F)");


    let mut unit = String::new();
    let celsius: String = String::from("C");
    let fahrenheit: String = String::from("F");

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
        let mut value_in_f: f64 = 0.;
        value_in_f = (to_convert as f64 * (9. / 5.)) + 32.;
        println!("{} degrees Celsius is equal to {:.2} degrees Fahrenheit", to_convert, value_in_f);
    }

    if unit.eq(&fahrenheit) {
        let mut value_in_c: f64 = 0.;
        value_in_c = (to_convert as f64 - 32.) * (5. / 9.);
        println!("{} degrees Fahrenheit is equal to {:.2} degrees Celsius", to_convert, value_in_c);
    }

}
*/



