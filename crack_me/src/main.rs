use std::env;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 
    {
        println!("Please enter an argument.");
        return;
    }

    let arg_one:&String = &args[1];

    let secret_number: u32 = 1550;

    match arg_one.trim().parse::<u32>() 
    {
        Ok(input_number) => {
            if input_number == secret_number {
                println!("You could be the next hacker...");
            } else {
                println!("Wrong!");
            }
        }
        Err(_) => println!("That's not a valid number."),
    }
}
