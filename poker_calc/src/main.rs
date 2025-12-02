use std::env;
fn main() {

    //Constants
    const DECK_SIZE: i32 = 52;
    const FLOP_SIZE: i32 = 5;
    const TURN_SIZE: i32 = 6;
    const RIVER_SIZE: i32 = 7;

    //Variables
    let args: Vec<String> = env::args().collect();
    let outs:&str  = &args[1];
    let pot:&str = &args[2];
    let bet:&str = &args[3];
    let cout_other_players:&str = &args[4];
    let stage: &str = &args[5];
    let mut card_odds: f32 = 0.0;
    let mut pot_odds: f32 = 0.0;

    
    
    if args.len() < 2 
    {
        println!("Please enter an argument.");
        return;
    }

    

    card_odds = calc_card_odds(FLOP_SIZE, outs, DECK_SIZE);
    println!("Card odds: {card_odds}");
    pot_odds = calc_pot_odds(bet, pot);
    println!("Pot odds: {pot_odds}");

    if card_odds < pot_odds 
    {
        println!("Good idea to call");            
    }
    else 
    {
        println!("Bad idea to call");
    }
}
fn calc_card_odds(stage: i32, outs: i32, deck_size: i32) -> f32
{
    let stage: f32 = stage as f32;
    let outs: f32 = outs as f32;
    let deck_size: f32 = deck_size as f32;
    let calc_odds: f32 = (deck_size - outs - stage) / outs;
    
    return calc_odds;
}
fn calc_pot_odds(bet: i32, pot_size: i32) -> f32
{
    let bet: f32 = bet as f32;
    let pot: f32 = pot_size as f32;
    let new_pot:f32 = bet + pot;

    let calc_odds: f32 = new_pot / bet;
    
    return calc_odds;   
}
fn parse_string_to_i32 (p_string: &str) -> i32
{
    match p_string.parse::<i32>() {

        Ok(num) => return num,
        Err(_) => 
        {
            eprintln!("Error: input must be a number");
            std::process::exit(1);
        }
    }
}