
pub enum FetchType {
    RealTime,
    Historical,
}

#[derive(Debug)]
pub struct StockData {
    pub name: String,
    pub open: String,
    pub close: String,
    pub now: String,
    pub high: String,
    pub low: String,
    pub buy: String,
    pub sell: String,
}

pub struct FetchRequest {
    pub code: String,
    pub fetch_type: FetchType,
}