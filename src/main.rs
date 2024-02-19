// Import the get_btc_price function from mod.rs
mod btc;
use btc::get_btc_price;

fn main() {
    // Get the price of Bitcoin in USD
    // If this fails, display an error

    let price_usd = get_btc_price()
        .expect("Failure in getting BTC price via API request");
        println!("BTC price in USD: {:.2}", price_usd);
}
