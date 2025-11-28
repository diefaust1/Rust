use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let one:&String  = &args[1];
    let two:&String = &args[2];
    let three:&String = &args[3];
    let four: &String = &args[4];
    
    println!("1: {one}");
    println!("2: {two}");
    println!("3: {three}");
    println!("4: {four}");
}
