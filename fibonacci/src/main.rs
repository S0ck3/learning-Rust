use std::io; // standard library for input/output

fn main() {
    
    println!("This program calculates the fibonacci sequence up to the number you enter.");
    println!("Enter a number: ");    
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // reading the input
        .expect("Error while reading input."); // error handling

    let num: i32 = match input
        .trim()                     // remove whitespaces at the beginning and end of the string
        .parse()                    // convert the input string to a number (i32)
    { Ok(num) => num, Err(_) => 0, };   // error handling
    
    let mut a = 1;
    let mut b = 1;
    let mut c = 0;

    println!("\n{} \n{}", a, b); 
    
    while a < num  && b < num  && c < num {
        
        c = a + b;
        a = b;
        b = c;
        
        if c < num {
            println!("{}", c);
        }
    }
    
}
