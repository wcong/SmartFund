use std::env;
use std::io::{stdout, Write};
pub mod data;
pub mod analyze;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <website> <stock> <algorithm>", args[0]);
        return Ok(());
    }

    let website = &args[1]; // First argument: website
    let stock = &args[2];   // Second argument: stock
    let algorithm = &args[3]; // Third argument: algorithm (rsi, macd, etc.)

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
        "alpha_vantage" => data::fetchdata::FetchRequest {
            code: stock.to_string(),
            fetch_type: data::fetchdata::FetchType::RealTime,
        }
        .fetch_alpha_vantage()
        .await?,
        _ => {
            eprintln!("Unsupported website: {}", website);
            return Ok(());
        }
    };

    // Example: Assume resp is Vec<StockData> and StockData has a close field as f64
    if algorithm == "rsi" {
        // Collect close prices from resp
        let closes: Vec<f64> = resp.iter()
            .filter_map(|s| s.close.parse::<f64>().ok())
            .collect();
        let rsi = analyze::rsi::calculate_rsi(&closes, 14);
        println!("RSI: {:?}", rsi);
    } else if algorithm == "macd" {
        // Collect close prices from resp
        let closes: Vec<f64> = resp.iter()
            .filter_map(|s| s.close.parse::<f64>().ok())
            .collect();
        if let Some((macd, signal, hist)) = analyze::macd::calculate_macd(&closes, 12, 26, 9) {
            println!("MACD: {:?}, Signal: {:?}, Histogram: {:?}", macd, signal, hist);
        } else {
            println!("MACD calculation failed or not enough data.");
        }
    } else {
        println!("{:?}", resp); // Default: print the StockData result
    }

    Ok(())
}