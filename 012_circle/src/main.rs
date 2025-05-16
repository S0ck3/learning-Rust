use std::io; // standard input/output library

const PI: f64 = std::f64::consts::PI; 

fn main() {
    
    println!("Please choose one of the following options: \n1. Radius \n2. Area \n3. Diameter");
    println!("Please enter your choice: ");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read input!");

    let num: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error in input: {}", e);
            return;
        }
    };


    let mut r = 0.0;
    let mut a = 0.0;
    let mut d = 0.0;
    
    match num {
        1 => {
            println!("Please enter the radius of the circle: ");
            let mut in1 = String::new();
            io::stdin().read_line(&mut in1).expect("Failed to read input!");
            
            r = match in1.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Invalid input: {}", e);
                    return;
                }
            };

            a = (r*r)*PI;
            d = 2.0*r;
        }
        2 => {
            println!("Please enter the area of the circle: ");
            let mut in2 = String::new();
            io::stdin().read_line(&mut in2).expect("Failed to read input!");
            a = match in2.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Invalid input: {}", e);
                    return;
                }
            };

            r = (a/PI).sqrt();            
            d = 2.0*r;
        }
        3 => {
            println!("Please enter the diameter of the circle: ");
            let mut in3 = String::new();
            io::stdin().read_line(&mut in3).expect("Failed to read input!");
            d = match in3.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Invalid input: {}", e);
                    return;
                }
            };

            r = d/2.0;
            a = (r*r)*PI;
        }
        _ => println!("Invalid input!")
    }
    
    if num <= 3 {
        println!("Radius: {:.2} \nArea: {:.2} \nDiameter: {:.2}", r, a, d);
    }
    
}

