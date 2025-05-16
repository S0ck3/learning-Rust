use std::io; // standard library for input/output

fn main() {
    println!("Please enter a word: ");

    let mut input = String::new(); // new string for the input
    io::stdin()
        .read_line(&mut input) // processing of the input
        .expect("Error while reading input."); // error handling

    let mut v = 0;
    let mut c = 0;
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']; // define vowels in vector

    for char in input.chars() {
        if char.is_alphabetic() {
            if vowels.contains(&char) {
                // checks if char is defined in vector "vowels"
                v += 1; // if yes, the vowel count is increased by 1
            } else {
                c += 1; // if no, the consonant count is increased by 1
            }
        }
    }

    println!("Vowel count: {}", v);
    println!("Consonant count: {}", c);
}
