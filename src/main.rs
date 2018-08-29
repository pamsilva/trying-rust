extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn a_random_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn play(val: u32) {
    loop {
        let mut guess = String::new();

        println!("Take your guess!");
        io::stdin().read_line(&mut guess)
            .expect("Failed to get input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
