
use anyhow::Result;
use ethers::prelude::Address;
use uniswap_api::client::Client;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<()> {
    // let client = Client::new("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")?;
    
    // // ETH/USDT pair on Uniswap V2
    // let pair = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".parse::<Address>()?;
    
    
    // println!("ETH/USDT Uniswap addr : {}", pair);
    
    // let mid_price = client.get_price(&pair).await?;
        
    // println!("ETH/USDT price: {:.6}", mid_price);
    
    Ok(())
}
