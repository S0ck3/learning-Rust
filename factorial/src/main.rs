use std::io; // standard library for input/output

fn main() {
    
    println!("This program calculates the factorial of a number.");
    println!("Please enter a number: ");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)// reading of the input
        .expect("Error while processing the input!"); // error handling
    
    let num: u64 = input
        .trim() // trim() removes all whitespace at the beginning and end of a string
        .parse()// convert input to u64 (unsigned 64 bit integer)
        .expect("That was not a number!"); // error handling
    
    // // call the calculate function and assign the result to the variable result
    let result = rechnen(num);
    
    // output of result
    println!("The factorial of {} is {}.", input, result);   
    
}

fn rechnen(num: u64) -> u64 {
    
    // checking if the number is 1 or less if yes the "loop" stops
    if num <= 1 {
        return 1;
    }
    
    // Fakultät von 5: 5 * 4 * 3 * 2 * 1 = 120
    num * rechnen(num - 1) // result returns automatically because there is no ; at the end of the line
    
}