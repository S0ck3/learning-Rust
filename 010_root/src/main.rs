fn main() {
    
    let mut sum: i32 = 0;
    for i in 1..101 {
        
        if i % 2 == 1 {
            sum += i*i;
        } else { continue; }
        
        println!("The 5th root of {} is {}", sum, fifth_root(sum as f64));
    }
}

fn fifth_root(a: f64) -> f64 {
    // cast 1/5 as f64 otherwise everything just resolves as 1 :(
    a.powf(1f64/5f64)
}
