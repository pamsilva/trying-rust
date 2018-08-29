extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn a_random_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn take_user_guess() -> u32 {
    println!("Take your guess:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to get input!");

        match guess.trim().parse() {
            Ok(val) => {
                return val;
            },
            Err(_) => {
                println!("Not a number, please insert a number:");
                continue;
            }
        };
    }
}

fn play(val: u32) {
    let mut counter : i8 = 0;
    loop {
        let guess = take_user_guess();

        counter = counter + 1;

        match guess.cmp(&val) {
            Ordering::Less => println!("Too small ..."),
            Ordering::Greater => println!("Too big ..."),
            Ordering::Equal => {
                println!("Well done !");
                println!("Success after {} attempts.", counter);
                break;
            }
        }
    }
}

fn main() {
    println!(" ***** Guess the number ******");
    println!(" ***** Between 1 and 100 *****");
    play(a_random_number());
}
