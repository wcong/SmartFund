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
use encoding_rs::GB18030;

lazy_static! {
    static ref NUM_PART: String = ",([\\.\\d]+)".repeat(29);
    static ref DATE_TIME_PART: String = ",([\\-\\.\\d:]+)".repeat(2);

    static ref GREP: String = format!(r"(\d+)=[^\s]([^\s,]+?){}{}", *NUM_PART, *DATE_TIME_PART);
    static ref GREP_WITH_PREFIX: String = format!(r"(\w{{2}}\d+)=[^\s]([^\s,]+?){}{}", *NUM_PART, *DATE_TIME_PART);

    static ref DELETE_NULL_REGEX: Regex = Regex::new(r#"(\w{2}\d+)=\"\";"#).expect("Invalid regex pattern");
    static ref GREP_REGEX: Regex = Regex::new(&GREP).expect("Invalid regex pattern");
    static ref GREP_WITH_PREFIX_REGEX: Regex = Regex::new(r#"var hq_str_(\w+)=["]([^"]+)["];"#).expect("Invalid regex pattern");
}

impl FetchRequest {
    pub async fn fetch_sina(&self) -> Result<Vec<StockData>, Box<dyn std::error::Error>> {
        let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    
        println!("{:#?}", time);
        let url = match self.fetch_type {
            FetchType::RealTime => format!("http://hq.sinajs.cn/rn={}&list={}", time, self.code),
            FetchType::Historical => format!("https://api.example.com/historical/{}", self.code),
        };
        
        println!("{:#?}", url);
        let headers = make_headers(url.as_str());
        let client = reqwest::Client::new();
        let request = client
            .get(url)
            .headers(headers)
            .build()?;
        
        println!("Request URL: {}", request.url());
        println!("Request Method: {}", request.method());
        println!("Request Headers: {:?}", request.headers());
        let response = client.execute(request).await?;
        
        println!("Response Status: {}", response.status());
        println!("Response Headers: {:?}", response.headers());
        
        // Retrieve the raw response body as bytes
        let response_bytes = response.bytes().await?;
        // Step 1: Decompress the GZIP-compressed response
        let mut gz_decoder = GzDecoder::new(&response_bytes[..]);
        let mut decompressed_bytes = Vec::new();
        gz_decoder.read_to_end(&mut decompressed_bytes)?;

       // Step 2: Decode the decompressed bytes using GB18030
       let (decoded_body, _, _) = GB18030.decode(&decompressed_bytes);
       println!("Decoded Response Body: {}", decoded_body);

       // Step 3: Use regex to parse the content
       let mut stock_data_list = Vec::new();

       // Split the response into lines and process each line
       for line in decoded_body.lines() {
           if let Some(cap) = GREP_WITH_PREFIX_REGEX.captures(line) {
               let identifier = cap.get(1).map_or("", |m| m.as_str());
               let data = cap.get(2).map_or("", |m| m.as_str());
               println!("Identifier: {}, Data: {}", identifier, data);

               // Split the data by ',' and collect into a list
               let data_list: Vec<&str> = data.split(',').collect();

               // Create a StockData instance and add it to the list
               let stock_data = StockData {
                   name: identifier.to_string(),
                   open: data_list.get(1).unwrap_or(&"").to_string(),
                   close: data_list.get(2).unwrap_or(&"").to_string(),
                   now: data_list.get(3).unwrap_or(&"").to_string(),
                   high: data_list.get(4).unwrap_or(&"").to_string(),
                   low: data_list.get(5).unwrap_or(&"").to_string(),
                   buy: data_list.get(6).unwrap_or(&"").to_string(),
                   sell: data_list.get(7).unwrap_or(&"").to_string(),
               };
               stock_data_list.push(stock_data);
           }
       }

        println!("{:#?}", stock_data_list);
        
        Ok(stock_data_list) 
    }
}