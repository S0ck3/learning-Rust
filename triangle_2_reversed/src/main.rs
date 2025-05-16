use std::io; // import standard input-output-library

fn main() {
    println!("This program prints a triangle out of * from 1 to the number you enter.");
    println!("Please enter a number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // reading input
        .expect("Error while reading the input!"); // error handling

    let num: i32 = match input
        .trim()                     // remove whitespaces at the beginning and end of the string
        .parse()                    // convert the input string to a number (i32)
    { Ok(num) => num, Err(_) => 0, };   // error handling
    
    println!(); // empty line after input (for looks)
    
    let mut a:i32 = 1;
    
    while a <= num{
        
        for _i in 1..=a{
            
            print!("*");
            
        }
        
        print!("\n");
        a += 1;
        
    }
}
