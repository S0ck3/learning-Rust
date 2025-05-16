// eingebundene Bibliotheken
use std::io; // für das lesen von Eingaben aus der Konsole


fn main() {
    
    println!("Bitte geben Sie eine Zahl ein: ");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)// lesen der Eingabe
        .expect("Fehler beim Lesen der Eingabe"); // fehlerbehandlung
    
    let num: u64 = input.trim().parse()// Eingabe in Zahl konvertieren
        .expect("Bitte geben Sie eine Zahl ein"); // fehlerbehandlung
    
    // aufrufen der rechnen-Funktion und zuweisen des Ergebnis zur Variable ergebnis
    let ergebnis = rechnen(num);
    
    // output vom Ergebnis
    println!("Die Fakultät von {} ist {}", input, ergebnis);   
    
}

fn rechnen(num: u64) -> u64 {
    
    // prüfen ob die "schleife" vorbei sein soll
    if num <= 1 {
        return 1;
    }
    
    // Fakultät von 5: 5 * 4 * 3 * 2 * 1 = 120
    num * rechnen(num - 1) // automatischer return vom ergebnis weil kein ; am ende ist
    
}