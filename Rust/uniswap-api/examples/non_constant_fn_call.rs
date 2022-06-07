use anyhow::Result;
use ethers::{
    abi::ethereum_types::Public,
    prelude::{
        k256::{ecdsa::SigningKey, SecretKey},
        Address, LocalWallet, Middleware, Signer, Wallet, H160, U256,
    },
};
use num_traits::Pow;
use rust_decimal::Decimal;
use std::sync::Arc;
use uniswap_api::client::Client;
use web3::ethabi::ethereum_types::Secret;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new("https://rinkeby.infura.io/v3/4f91b5fc02b444ef84bceee5d8ff232a")?;
    let dai_address = "0xc7ad46e0b8a400bb3c915120d284aafba8fc4735".parse::<Address>()?;
    let weth_addr = "0xc778417e063141139fce010982780140aa0cd5ab".parse::<Address>()?;
    let router02_addr = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    let factory_addr = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;

    let weth_dai_price = client
        .get_price(&factory_addr,  &weth_addr, &dai_address)
        .await?;

    println!("weth/dai price : {}", weth_dai_price);

    let chain_id = client.provider().get_chainid().await?.as_u64();
    let wallet = "408561c191f6970e10d543214f55861cd11e0cbd035aa636f16a1a4a00e1b4fb"
        .parse::<LocalWallet>()?;
    println!("wallet chain id: {}", wallet.chain_id());

    client.set_wallet(wallet).await?;
    let amount_out_min = Decimal::from(5 as u32);

    let receipt = client
        .swap_exact_eth_for_tokens(&router02_addr, amount_out_min, &[weth_addr, dai_address])
        .await?;

    println!(
        "tx: {:?} confirmed, execution successful?: {:?}",
        receipt.transaction_hash, receipt.status
    );

    Ok(())
}
