use regex::Regex;
use crate::data::fetchdata::FetchRequest;
use crate::data::fetchdata::FetchType;
use crate::data::fetchdata::StockData;
use crate::data::util::make_headers;
use serde_json::Value;
use lazy_static::lazy_static;
use flate2::read::GzDecoder;
use std::io::Read;
use encoding_rs::UTF_8;

lazy_static! {
    static ref TENCENT_REGEX: Regex = Regex::new(r#"v_(\w+)="([^"]+)";"#).expect("Invalid regex pattern");
}
// GHNY1SD4BGWVIL06
impl FetchRequest {
    pub async fn fetch_alpha_vantage(&self) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
        let url = match self.fetch_type {
            FetchType::RealTime => format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}", self.code, "GHNY1SD4BGWVIL06"),
            FetchType::Historical => format!("https://api.example.com/historical/{}", self.code),
        };

        let headers = make_headers(url.as_str());
        let client = reqwest::Client::new();
        let response = client.get(url).headers(headers).send().await?;

        // Check if the response is gzip encoded
        let is_gzip = response
            .headers()
            .get("content-encoding")
            .map_or(false, |v| v.to_str().unwrap_or("").to_lowercase().contains("gzip"));

        let resp_text = if is_gzip {
            // Retrieve the raw response body as bytes
            let response_bytes = response.bytes().await?;
            // Decompress the GZIP-compressed response
            let mut gz_decoder = GzDecoder::new(&response_bytes[..]);
            let mut decompressed_bytes = Vec::new();
            gz_decoder.read_to_end(&mut decompressed_bytes)?;
            // Decode to UTF-8 string
            let (decoded_body, _, _) = UTF_8.decode(&decompressed_bytes);
            decoded_body.into_owned()
        } else {
            response.text().await?
        };
        
        println!("Decoded Response Body: {}", resp_text);

        let json: Value = serde_json::from_str(&resp_text)?;

        // Step 3: Use regex to parse the content
        let mut stock_data_list = Vec::new();

         if let Some(series) = json.get("Time Series (Daily)") {
            // Collect and sort dates descending (latest first)
            let mut days: Vec<_> = series.as_object().unwrap().iter().collect();
            days.sort_by(|a, b| b.0.cmp(a.0)); // descending order

            for (date, day_data) in days.iter() {
                let open = day_data.get("1. open").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let high = day_data.get("2. high").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let low = day_data.get("3. low").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let close = day_data.get("4. close").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let volume = day_data.get("5. volume").and_then(|v| v.as_str()).unwrap_or("").to_string();

                let stock_data = StockData {
                    // Add fields as defined in your struct
                    name: self.code.to_string(),
                    open,
                    high,
                    low,
                    close,
                    now: date.to_string(), // Assuming 'now' is the date
                    sell: date.to_string(),
                    buy: volume.clone(), // Assuming buy is volume for simplicity
                };
                stock_data_list.push(stock_data);
            }
        }

        println!("{:#?}", stock_data_list);

        Ok(stock_data_list)
    }
}