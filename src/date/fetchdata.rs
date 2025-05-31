use http::header::USER_AGENT;
use http::header::ACCEPT_ENCODING;

enum FetchType {
    RealTime,
    Historical,
}

struct FetchData {
    pub code: String,
    pub fetchType: FetchType,
}

struct FetchRequest {
    pub fetchData: FetchData,
}

impl FetchRequest {
    pub fn new(code: String, fetchType: FetchType) -> Self {
        Self {
            fetchData: FetchData { code, fetchType },
        }
    }

    pub async fn fetch(&self) -> Result<(), Box<dyn std::error::Error>> {
        let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
        let url = match self.fetchData.fetchType {
            FetchType::RealTime => format!("http://hq.sinajs.cn/rn={}&list={}", time, self.fetchData.code),
            FetchType::Historical => format!("https://api.example.com/historical/{}", self.fetchData.code),
        };

        let resp = reqwest::get(&url).await?;
        let data: serde_json::Value = resp.json().await?;
        
        println!("{:#?}", data);
        Ok(()) 
    }

    fn MakeHeaders() -> http::header::HeaderMap {
        let mut headers = http::header::HeaderMap::new();
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip, deflate, sdch".parse().unwrap());
        headers
    }
}