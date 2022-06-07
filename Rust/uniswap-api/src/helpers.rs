use anyhow::{Result, anyhow, Ok};
use async_trait::async_trait;
use ethers::{
    abi::Detokenize,
    prelude::{builders::ContractCall, JsonRpcClient, Middleware, Provider, U256},
};
use num_traits::{ToPrimitive, FromPrimitive};
use rust_decimal::Decimal;
use std::time::{SystemTime, UNIX_EPOCH};

#[async_trait]
pub trait ContractCallHelper<P>
where
    Self: Sized,
    P: JsonRpcClient,
{
    async fn fill_gas_fields(self, provider: &Provider<P>) -> Result<Self>;
}

#[async_trait]
impl<M, D, P>  ContractCallHelper<P> for ContractCall<M, D>
where
    M: Middleware + 'static,
    D: Detokenize + Send + Sync + 'static,
    P: JsonRpcClient 
{
    async fn fill_gas_fields(self, provider: &Provider<P>) -> Result<Self> {
    
        let est_gas = &self.estimate_gas().await?;
        let gas_price = provider.get_gas_price().await?;
        
        Ok(self.gas_price(gas_price).gas(est_gas))
    }
}

pub(crate) fn get_valid_timestamp(future_millis: u128) -> U256 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let time_millis = since_epoch.as_millis().checked_add(future_millis).unwrap();

    U256::from(time_millis)
}

pub(crate) fn calculate_price(token_input_reserve: u128, token_output_reserve: u128, token_input_dec: u8, token_output_dec: u8) -> Result<Decimal> {
    let f_res0 = token_input_reserve.to_f64().ok_or(anyhow!("failed to convert res0 to f64"))?;
    let f_res1 = token_output_reserve.to_f64().ok_or(anyhow!("failed to convert res1 to f64"))?;
    
   

    let f_price = f_res1 / f_res0 * f64::powi(10.0, (token_input_dec - token_output_dec) as i32);
    
    
    // println!("{:?}", f_price);
    
    Ok(Decimal::from_f64(f_price).ok_or(anyhow!("failed to convert price from f64 to Decimal"))?)
}
