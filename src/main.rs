use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let tries = 10;
    println!("Guess a number!! you have {tries} tries");

    let answer = rand::thread_rng().gen_range(1..=100);

    if rand::random() {
        println!("psst! -> {answer}");
    }

    let mut counter = 1;
    // you can return value from a while loop!
    let wth = loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            break "Quit";
        }

        // match is like the switch case, but you can use it anywhere!
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        let remaining = tries - counter;
        match guess.cmp(&answer) {
            Ordering::Less => {
                println!("Too small! {remaining} guesses left.");
                counter += 1;
            }
            Ordering::Greater => {
                println!("Too big! {remaining} guesses left.");
                counter += 1;
            }
            Ordering::Equal => {
                println!("You are correct!!!! you get a fuzzy feeling in your heart.");
                break "Win";
            }
        };

        if counter > tries {
            break "Lose";
        }
    };
    println!("----------\nYou {wth}!")
}
