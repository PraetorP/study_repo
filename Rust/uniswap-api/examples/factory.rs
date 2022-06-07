
use anyhow::Result;
use ethers::prelude::Address;
use uniswap_api::client::Client;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")?;
    
    
    let factory = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;
    let router = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    let btc = "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599".parse::<Address>()?;
    let weth = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse::<Address>()?;
    
    let usdt = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse::<Address>()?;
    
    
    let pair = client.get_pair_address(&factory, &btc, &usdt).await?;
    
    let mid_price = client.get_router_price(&factory, &router, &weth, &usdt).await?;
    println!("WETH/USDT price based on router value: {:.6}", mid_price);
    
    let mid_price = client.get_price(&factory, &weth, &usdt).await?;
    println!("WETH/USDT price based on reserve: {:.6}", mid_price);
    

    println!("\nBTC/USDT Uniswap pair addr : {}\n", pair);
    
    
    let mid_price = client.get_router_price(&factory, &router, &btc, &usdt).await?;
        
    println!("BTC/USDT price based on router_value: {:.6}", mid_price);
    
    
    let mid_price = client.get_price(&factory, &btc, &usdt).await?;
        
    println!("BTC/USDT price based on reserve: {:.6}", mid_price);
    
    
    
    
    Ok(())
}
