use std::io;

fn main() {

    println!("Bitte geben Sie ein Wort ein: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // lesen der Eingabe
        .expect("Fehler beim Lesen der Eingabe."); // fehlerbehandlung

    let mut vokal = 0;
    let mut konsonant = 0;
        
    for buchstabe in input.chars() {
        match buchstabe {
            'a' => vokal += 1,
            'b' => konsonant += 1,
            'c' => konsonant += 1,
            'd' => konsonant += 1,
            'e' => vokal += 1,
            'f' => konsonant += 1,
            'g' => konsonant += 1,
            'h' => konsonant += 1,
            'i' => vokal += 1,
            'j' => konsonant += 1,
            'k' => konsonant += 1,
            'l' => konsonant += 1,
            'm' => konsonant += 1,
            'n' => konsonant += 1,
            'o' => vokal += 1,
            'p' => konsonant += 1,
            'q' => konsonant += 1,
            'r' => konsonant += 1,
            's' => konsonant += 1,
            't' => konsonant += 1,
            'u' => vokal += 1,
            'v' => konsonant += 1,
            'w' => konsonant += 1,
            'x' => konsonant += 1,
            'y' => konsonant += 1,
            'z' => konsonant += 1,
            'ä' => vokal += 1,
            'ö' => vokal += 1,
            'ü' => vokal += 1,
            'A' => vokal += 1,
            'B' => konsonant += 1,
            'C' => konsonant += 1,
            'D' => konsonant += 1,
            'E' => vokal += 1,
            'F' => konsonant += 1,
            'G' => konsonant += 1,
            'H' => konsonant += 1,
            'I' => vokal += 1,
            'J' => konsonant += 1,
            'K' => konsonant += 1,
            'L' => konsonant += 1,
            'M' => konsonant += 1,
            'N' => konsonant += 1,
            'O' => vokal += 1,
            'P' => konsonant += 1,
            'Q' => konsonant += 1,
            'R' => konsonant += 1,
            'S' => konsonant += 1,
            'T' => konsonant += 1,
            'U' => vokal += 1,
            'V' => konsonant += 1,
            'W' => konsonant += 1,
            'X' => konsonant += 1,
            'Y' => konsonant += 1,
            'Z' => konsonant += 1,
            'Ä' => vokal += 1,
            'Ö' => vokal += 1,
            'Ü' => vokal += 1,
            'ß' => konsonant += 1,
            _ => continue,
        }
    }
    
    println!("Vokale: {}", vokal);
    println!("Konsonanten: {}", konsonant);
}
