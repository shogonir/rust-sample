extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // at first
    println!("Guess the number!");
    let mut count = 0;

    // generate secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // receive input number
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
        count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You won with {} answers.", count);
                break;
            }
        }
    }
}
