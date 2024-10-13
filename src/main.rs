use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guessed it right!");
                break;
            }
        }
    }
}

/*
    https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

    variables
        let apples = 5;
    variables in rust are immutable by default
    to make them mutable, we must use:
        let mut bananas = 5;

    String::new is a function that returns a new instance of a String (a growable UTF-8 encoded bit of text)
    :: syntax indicates that "new" is an associated function of the String type

    the stdin function allows us to handle user input
    it would be possible to call std::io::stdin instead of declaring use std::io in the beginning and than calling io::stdin

    .read_line reads the user input and stores it in the String passed as an argument
    the & in the argument indicates that it is a REFERENCE, which lets multiple parts of the code access that piece of data without having to copy it into memory multiple times
    references are also immutable by default, therefore we need to use &mut guess instead of &guess
    read_line returns Result, an enum, which can be Ok or err.
    Ok contains the successfully generated value.
    err cointains information about why it failed.
    Instances of Result contain the "expect" method.
    If this instance of Result is an err, expect will crash the program and show the massege passed as argument.
    If it is Ok, it will take the return value and return it so we can use it.
    See a possible warning in the documentation.

    
*/