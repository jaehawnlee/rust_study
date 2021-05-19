extern crate rand;

use std::io;
use std::cmp::Ordering;
//use std::string::ToString;
use rand::Rng;

fn main() 
{
    println!("Guess the number!");

    let secret_number =  rand::thread_rng().gen_range(1..100);

    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();
    loop 
    {
        guess.clear();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess:u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => 
            {
                println!("Wrong number format, {}", guess);
                continue;
            }
        };

        match guess.cmp(&secret_number) 
        {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big! {} / {}",guess, secret_number),
            Ordering::Equal   => 
            {
                println!("You win!");
                break;
            }
        }
    }
}
