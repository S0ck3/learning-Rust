use std::io; // standard library for input/output

const EURO_TO_DOLLAR: f64 = 1.12;
const DOLLAR_TO_EURO: f64 = 0.89;
struct Currency {
    sign: String,
    converted_sign: String,
}

fn main() {

    println!("Choose 1 for euro to dollar conversion or 2 for dollar to euro conversion.");
    println!("Enter your choice: ");

    let mut input_choice = String::new();
    io::stdin()
        .read_line(&mut input_choice) // reading the input
        .expect("Error while reading input."); // error handling

    let num: i32 = match input_choice
        .trim()                     // remove whitespaces at the beginning and end of the string
        .parse()                    // convert the input string to a number (i32)
    { Ok(num) => num, Err(_) => 0, };   // error handling

    if num == 1 || num == 2 {
        convert(num);
    } else {
        println!("Invalid input.");
    }

}

fn convert(choice: i32){

    let euro = Currency {
        sign: "€".to_string(),
        converted_sign: "$".to_string(),
    };

    let dollar = Currency {
        sign: "$".to_string(),
        converted_sign: "€".to_string(),
    };

    println!("Enter the amount you want to convert: ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // reading the input
        .expect("Error while reading input."); // error handling

    let num: f64 = match input
        .trim()                     // remove whitespaces at the beginning and end of the string
        .parse()                    // convert the input string to a number (i32)
    { Ok(num) => num, Err(_) => 0.0, };   // error handling

    if choice == 1 {
        println!("{:.2} {} = {} {:.2}", num, euro.sign, euro.converted_sign, num * EURO_TO_DOLLAR);
    } else if choice == 2 {
        println!("{} {:.2} = {:.2} {}", dollar.sign, num, num * DOLLAR_TO_EURO, dollar.converted_sign);
    } else {
        println!("Invalid input.");
    }

}