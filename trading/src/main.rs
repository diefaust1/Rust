use yfinance_rs::{Interval, Range, Ticker, YfClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let client = YfClient::default();
    let ticker = Ticker::new(&client, "BMW");

    // Get the latest quote
    let quote = ticker.quote().await?;
    println!("Latest price for AAPL: ${:.2}", quote.price.as_ref().map(|p| yfinance_rs::core::conversions::money_to_f64(p)).unwrap_or(0.0));

    Ok(())
}
