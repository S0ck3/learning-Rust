use rand::Rng; // library for random numbers
use std::io; // standard library for input-output

fn main() {

    // Generate a random number between 1 and 100 (inclusive)
    let target: i32 = rand::thread_rng().gen_range(1..101);
    
    let mut guesses = 0;
    let mut num = input();
    
    while num != target {
        
        if num > target{
            println!("\n=> The wanted number is smaller!")
        }
        
        if num < target {
            println!("\n=> The wanted number is bigger!")
        }
        
        guesses += 1;
        num = input();
        
    }
    
    println!("\nYou won! The magic number was: {}", target);
    println!("You got it after {} guesses.", guesses);
    
}

fn input() -> i32 {

    println!("\nPlease enter your guess: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // reading the input
        .expect("Error while reading input."); // error handling

    let num: i32 = match input
        .trim()                     // remove whitespaces at the beginning and end of the string
        .parse()                    // convert the input string to a number (i32)
            { Ok(num) => num, Err(_) => 0, };   // error handling
    
    num
    
}
