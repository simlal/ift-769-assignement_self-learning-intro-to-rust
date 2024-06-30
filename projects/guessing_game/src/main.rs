use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Type infered from the match expression
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Answer is {}", secret_num);
    
    // Get the guess as a mutable string
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // Convert to u32 with shadowing
    let guess: u32 = guess.trim().parse().expect("Please type an interger!");
    println!("You guessed: {}", guess);

    // Get the result
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }

}