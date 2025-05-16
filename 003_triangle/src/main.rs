
use std::io; // aufrufen der standard input/output bibliothek

fn main() {
    
    println!("Wie breit soll die Basis der Pyramide sein: ");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)// lesen der Eingabe
        .expect("Fehler beim Lesen der Eingabe"); // fehlerbehandlung

    let num: u64 = input.trim().parse()// Eingabe in Zahl konvertieren
        .expect("Bitte geben Sie eine Zahl ein"); // fehlerbehandlung
    
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
