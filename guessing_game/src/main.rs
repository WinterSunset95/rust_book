




use std::io;
use rand::Rng;
use colored::*;

fn main() {

    println!("Guessing game!!");

    let number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: i32 = guess.trim().parse().expect("Please type a number!");

        if guess > number {
            println!("{}", "Too High!".red());
        } else if guess < number {
            println!("{}", "Too Low!".red());
        } else {
            println!("{}", "You Win!".green());
            break
        }
    }
}
