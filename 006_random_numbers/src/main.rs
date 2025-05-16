use rand::Rng;  // library for random numbers

fn main() {

    let num: i32 = rand::thread_rng().gen_range(0..3); // random number between 0 and 3
    
    match num{                          // match the random number to a function
        0 => println!("{}", first()),
        1 => println!("{}", second()),
        2 => println!("{}", third()), 
        _ => println!("I don't know what to do here. But you wont see this i hope. :3"),
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