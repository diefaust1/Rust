use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let one:&String  = &args[1];
    let two:&String = &args[2];
    
    
    println!("1: {one}");
    println!("2: {two}");
}
