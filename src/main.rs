use std::io::{stdout, Write};
pub mod data;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = data::fetchdata::FetchRequest {
        code: "s_sh000001,s_sz399001,nf_IF0,rt_hkHSI,gb_$dji,gb_ixic,b_SX5E,b_UKX,b_NKY,hf_CL,hf_GC,hf_SI,hf_CAD".to_string(),
        fetch_type: data::fetchdata::FetchType::RealTime,
        }
        .fetch_sina()
        .await?;
    println!("{:?}", resp); // Handle or print the StockData result
    Ok(())
}