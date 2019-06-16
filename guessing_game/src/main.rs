use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //Generating a secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    //Processing a guess
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed {}", guess);
}
