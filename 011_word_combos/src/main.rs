use std::io; // standard library input/output

fn main() {

    println!("Please enter a word with 2 - 4 letters: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // reading the input
        .expect("Error while reading input."); // error handling

        let word = input.trim(); // removing whitespaces at the beginning and end of the string

        //if word.len() = 4 {
        let chars: Vec<char> = word.chars().collect(); // converting the string to a vector of chars

    match chars.len() {
        2 => {
            // 2! = 2 permutations
            println!("Every possible combination with 2 letters:");
            println!("{}{}", chars[0], chars[1]);
            println!("{}{}", chars[1], chars[0]);
        },
        3 => {
            // 3! = 6 permutations
            println!("Every possible combination with 3 letters:");
            println!("{}{}{}", chars[0], chars[1], chars[2]);
            println!("{}{}{}", chars[0], chars[2], chars[1]);
            println!("{}{}{}", chars[1], chars[0], chars[2]);
            println!("{}{}{}", chars[1], chars[2], chars[0]);
            println!("{}{}{}", chars[2], chars[0], chars[1]);
            println!("{}{}{}", chars[2], chars[1], chars[0]);
        },
        4 => {
            // 4! = 24 permutations
            println!("Every possible combination with 4 letters:");
            println!("{}{}{}{}",chars[0],chars[1],chars[2],chars[3]);
            println!("{}{}{}{}",chars[0],chars[1],chars[3],chars[2]);
            println!("{}{}{}{}",chars[0],chars[2],chars[1],chars[3]);
            println!("{}{}{}{}",chars[0],chars[2],chars[3],chars[1]);
            println!("{}{}{}{}",chars[0],chars[3],chars[1],chars[2]);
            println!("{}{}{}{}",chars[0],chars[3],chars[2],chars[1]);

            println!("{}{}{}{}",chars[1],chars[0],chars[2],chars[3]);
            println!("{}{}{}{}",chars[1],chars[0],chars[3],chars[2]);
            println!("{}{}{}{}",chars[1],chars[2],chars[0],chars[3]);
            println!("{}{}{}{}",chars[1],chars[2],chars[3],chars[0]);
            println!("{}{}{}{}",chars[1],chars[3],chars[0],chars[2]);
            println!("{}{}{}{}",chars[1],chars[3],chars[2],chars[0]);

            println!("{}{}{}{}",chars[2],chars[0],chars[1],chars[3]);
            println!("{}{}{}{}",chars[2],chars[0],chars[3],chars[1]);
            println!("{}{}{}{}",chars[2],chars[1],chars[0],chars[3]);
            println!("{}{}{}{}",chars[2],chars[1],chars[3],chars[0]);
            println!("{}{}{}{}",chars[2],chars[3],chars[0],chars[1]);
            println!("{}{}{}{}",chars[2],chars[3],chars[1],chars[0]);

            println!("{}{}{}{}",chars[3],chars[0],chars[1],chars[2]);
            println!("{}{}{}{}",chars[3],chars[0],chars[2],chars[1]);
            println!("{}{}{}{}",chars[3],chars[1],chars[0],chars[2]);
            println!("{}{}{}{}",chars[3],chars[1],chars[2],chars[0]);
            println!("{}{}{}{}",chars[3],chars[2],chars[0],chars[1]);
            println!("{}{}{}{}",chars[3],chars[2],chars[1],chars[0]);
        },
        _ => {
            println!("The word must have 2, 3 or 4 letters!");
        }
    }




}
