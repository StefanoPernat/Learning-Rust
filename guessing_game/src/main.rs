use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //Generating a secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    //Create a loop for asking multiple guess
    loop {
        //Processing a guess
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //Convert the guess to a number - handling invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue
        };
        
        println!("You guessed {}", guess);

        //Compere user guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                //Exit from the loop when guess is correct
                println!("You win!");
                break;
            },
        }
    }
}
