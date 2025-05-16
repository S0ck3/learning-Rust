use rand::Rng;  // standardbibliothek für zufallsgeneratoren

fn main() {
        
    let mut rng = rand::thread_rng(); // Zufallsgenerator erzeugen
    
    let zahl: i32 = rng.gen_range(0..3);
    
    match zahl{
        0 => println!("{}", first()),
        1 => println!("{}", second()),
        2 => println!("{}", third()), 
        _ => println!("I don't know what to do."),
    }
}

fn first() -> String{
    "I'm the first function.".to_string()
}
fn second() -> String{
    "I am number two.".to_string()
}
fn third() -> String{
    "I am the third function :3".to_string()    
}