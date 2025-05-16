use rand::Rng; // standardbibliothek für zufallsgeneratoren
use std::io; // standardbibliothek input-output

fn main() {

    let mut rng = rand::thread_rng(); // Zufallsgenerator erzeugen
    
    let zahl: i32 = rng.gen_range(1..101);
    
    let mut num = eingabe();
    
    while num != zahl {
        
        if num > zahl{
            println!("\nDie gesuchte Zahl ist kleiner!")
        }
        
        if num < zahl {
            println!("\nDie gesuchte Zahl ist größer!")
        }
        
        num = eingabe();
        
    }
    
    println!("\nDu hast gewonnen! Die gesuchte zahl war: {}", zahl)
    
}

fn eingabe() -> i32 {

    println!("\nWas denken Sie ist die gesuchte Zahl: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // lesen der Eingabe
        .expect("Fehler beim Lesen der Eingabe."); // fehlerbehandlung

    let num: i32 = input
        .trim()
        .parse() // Eingabe in Zahl konvertieren
        .expect("Bitte geben Sie eine gültige Zahl ein!"); // fehlerbehandlung
    
    num
    
}
