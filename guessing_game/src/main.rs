use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret}");
    println!("Please enter your guess");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Oops, something went wrong :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Guess is {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("Ya guessed it!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
        }
    }
}
