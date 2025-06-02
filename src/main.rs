use std::env;
use std::io::{stdout, Write};
pub mod data;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <website> <stock>", args[0]);
        return Ok(());
    }

    let website = &args[1]; // First argument: website
    let stock = &args[2];   // Second argument: stock

    // Fetch data based on the website
    let resp = match website.as_str() {
        "sina" => data::fetchdata::FetchRequest {
            code: stock.to_string(),
            fetch_type: data::fetchdata::FetchType::RealTime,
        }
        .fetch_sina()
        .await?,
        "tencent" => data::fetchdata::FetchRequest {
            code: stock.to_string(),
            fetch_type: data::fetchdata::FetchType::RealTime,
        }
        .fetch_tencent()
        .await?,
        _ => {
            eprintln!("Unsupported website: {}", website);
            return Ok(());
        }
    };

    println!("{:?}", resp); // Handle or print the StockData result
    Ok(())
}