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
    loop {
        let guess = take_user_guess();

        match guess.cmp(&val) {
            Ordering::Less => println!("Too small ..."),
            Ordering::Greater => println!("Too big ..."),
            Ordering::Equal => {
                println!("Well done !");
                break;
            }
        }
    }
}

fn main() {
    println!("Hello, POTATO!");
    play(a_random_number());
}
