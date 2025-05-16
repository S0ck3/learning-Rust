use std::io; // import standard input-output-library

fn main() {
    println!("Bitte geben Sie die Breite des Dreiecks an: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // lesen der Eingabe
        .expect("Fehler beim Lesen der Eingabe."); // fehlerbehandlung

    let num: i32 = input
        .trim()
        .parse() // Eingabe in Zahl konvertieren
        .expect("Bitte geben Sie eine gültige Zahl ein!"); // fehlerbehandlung

    println!(); // für leerzeile nach eingabe (kosmetik)

    let mut a:i32 = 1;

    while a <= num{
        
        // wenn a eine gerade zahl ist dann überspringen
        if a % 2 == 0{
            a += 1;
            continue;
        }
        
        for _i in 1..=a{

            print!("*");

        }

        print!("\n");
        a += 1;

    }
}