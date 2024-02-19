I am working on a Rust project.
I have two files in the src folder:
`btc.rs`
`main.rs`

This is what btc.rs looks like:
```rust

// To deserialize the JSON response
#[derive(Deserialize)]
struct BTCPrice {
    // As the JSON data type has "USD" as the field name.
    // So, the struct `BTCPrice` must have the field with the same name.
    // In a production code scenario, Rust recommends snake_case.
    // This is why "usd" is in lowercase, which can be treated as uppercase,
    // or any custom name as provided in `rename` field.
    #[serde(rename = "USD")]
    usd: f32,
}

#[tokio::main]
pub (crate) async fn get_btc_price() -> Result<f32> {
    // path has to be w.r.t `Cargo.toml` file
    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = env::var("URL").expect("URL var not found");

    let body = reqwest::get(url).await?.json::<BTCPrice>().await?;

    let price_usd = body.usd;

    Ok(price_usd)

}

```

This is what my main.rs looks like:
```rust

fn main() {
    // Get the price of Bitcoin in USD
    // If this fails, display an error

    let price_usd = get_btc_price()
        .expect("Failure in getting BTC price via API request");
        println!("BTC price in USD: {:.2}", price_usd);
}

```

How can I use the get_btc_price function in main.rs?
