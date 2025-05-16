use std::io; // standard library for input/output

fn main() {
    println!("This program prints a triangle out of * from the number you enter down to 1.");
    println!("Please enter a number: ");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)// reading input
        .expect("Error while reading input!"); // error handling

    let num: i32 = match input
        .trim()                     // remove whitespaces at the beginning and end of the string
        .parse()                    // convert the input string to a number (i32)
        { Ok(num) => num, Err(_) => 0, };   // error handling
    
    let mut a = num;
    
    while a > 0 {
        
        let mut b = a;
        
        while b > 0 {
            
            print!("*");
            b -= 1;
        }
        
        print!("\n");
        a -= 1;        
    }
}
