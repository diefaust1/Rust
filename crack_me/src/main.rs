use std::io;

fn main() 
{
    let secret_number: u32 = 1550;

    println!("Password?");

    loop 
    {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => 
            {
                println!("Wrong Password");
                continue;
            }
        };

        if guess == secret_number
        {
            println!("You have potential to be a hacker...");
            break;
        }else 
        {
            println!("Wrong Password")
        }
    }
}
