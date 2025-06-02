use regex::Regex;
use crate::data::fetchdata::FetchRequest;
use crate::data::fetchdata::FetchType;
use crate::data::fetchdata::StockData;
use crate::data::util::make_headers;
use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use std::io::Read;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use encoding_rs::GBK;

lazy_static! {
    static ref TENCENT_REGEX: Regex = Regex::new(r#"v_(\w+)="([^"]+)";"#).expect("Invalid regex pattern");
}

impl FetchRequest {
    pub async fn fetch_tencent(&self) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let url = match self.fetch_type {
            FetchType::RealTime => format!("http://qt.gtimg.cn/q={}", self.code),
            FetchType::Historical => format!("https://api.example.com/historical/{}", self.code),
        };

        let headers = make_headers(url.as_str());
        let client = reqwest::Client::new();
        let response = client.get(url).headers(headers).send().await?;

        // Retrieve the raw response body as bytes
        let response_bytes = response.bytes().await?;

        // Step 1: Decompress the GZIP-compressed response
        let mut gz_decoder = GzDecoder::new(&response_bytes[..]);
        let mut decompressed_bytes = Vec::new();
        gz_decoder.read_to_end(&mut decompressed_bytes)?;

         // Step 2: Decode the decompressed bytes using GBK
         let (decoded_body, _, _) = GBK.decode(&decompressed_bytes);
         println!("Decoded Response Body: {}", decoded_body);

        // Step 3: Use regex to parse the content
        let mut stock_data_list = Vec::new();

        // Iterate over all matches in the response body
        for cap in TENCENT_REGEX.captures_iter(&decoded_body) {
            let identifier = cap.get(1).map_or("", |m| m.as_str());
            let data = cap.get(2).map_or("", |m| m.as_str());
            println!("Identifier: {}, Data: {}", identifier, data);

            // Split the data by '~' and collect into a list
            let data_list: Vec<&str> = data.split('~').collect();

            // Create a StockData instance and add it to the list
            let stock_data = StockData {
                name: identifier.to_string(),
                open: data_list.get(3).unwrap_or(&"").to_string(),
                close: data_list.get(4).unwrap_or(&"").to_string(),
                now: data_list.get(5).unwrap_or(&"").to_string(),
                high: data_list.get(6).unwrap_or(&"").to_string(),
                low: data_list.get(7).unwrap_or(&"").to_string(),
                buy: data_list.get(8).unwrap_or(&"").to_string(),
                sell: data_list.get(9).unwrap_or(&"").to_string(),
            };
            stock_data_list.push(stock_data);
        }

        println!("{:#?}", stock_data_list);

        Ok(stock_data_list)
    }
}