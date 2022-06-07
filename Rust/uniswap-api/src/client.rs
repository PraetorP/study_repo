use std::ops::Add;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::Duration;
use ethers::prelude::*;
use rust_decimal::prelude::*;

use num_traits::Pow;

use crate::bindings::{IUniswapV2Router02, UniswapFactory, UniswapV2Pair, ERC20};
use crate::helpers::{ContractCallHelper, calculate_price};


pub struct Client<P> {
    provider: Arc<Provider<P>>,
    wallet: Option<LocalWallet>,
    signer: Option<SignerMiddleware<Arc<Provider<P>>, LocalWallet>>,
}
impl Client<Http> {
    pub fn new(endpoint: &str) -> Result<Self> {
        let client = Provider::<Http>::try_from(endpoint)?;
        Ok(Client {
            provider: Arc::new(client),
            wallet: None,
            signer: None,
        })
    }
    // pub async fn get_price(&self, uniswap_pair: &Address) -> Result<Decimal> {

    //     let pair = IUniswapV2Pair::new(*uniswap_pair, Arc::clone(&self.transport));

    //     let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
    //     println!("Reserves (ETH, USDT): ({}, {})", reserve0, reserve1);

    //     let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;

    //     let dec_res0 = Decimal::from(reserve0);
    //     let dec_res1 = Decimal::from(reserve1);

    //     let price = (dec_res1 / dec_res0) * Decimal::from(1_000_000_000_000 as u64);

    //     Ok(price)
    // }

    // /// Set the client's wallet.
    // pub fn set_wallet(&mut self, wallet: LocalWallet) {
    //     self.wallet = Some(wallet);
    // }

    // /// Get a reference to the client's wallet.
    // pub fn wallet(&self) -> Result<&LocalWallet> {
    //     self.wallet
    //     .as_ref()
    //     .ok_or(anyhow!("wallet isn't provided"))
    // }
}

impl<P: JsonRpcClient + 'static> Client<P> {
    /// Get a reference to the client's provider.
    pub fn provider(&self) -> &Provider<P> {
        self.provider.as_ref()
    }

    /// Set the client's wallet.
    pub async fn set_wallet(&mut self, mut wallet: LocalWallet) -> Result<()> {
        wallet = wallet.with_chain_id(self.provider.get_chainid().await?.as_u64());

        self.signer = Some(SignerMiddleware::<Arc<Provider<P>>, LocalWallet>::new(
            self.provider.clone(),
            wallet.clone(),
        ));
        self.wallet = Some(wallet);
        Ok(())
    }
    /// Get a reference to the client's wallet.
    pub fn wallet(&self) -> Result<&LocalWallet> {
        self.wallet.as_ref().ok_or(anyhow!("wallet isn't provided"))
    }

    /// Set the client's signer.
    pub fn set_signer(&mut self, signer: SignerMiddleware<Arc<Provider<P>>, LocalWallet>) {
        self.signer = Some(signer);
    }

    /// Get a reference to the client's signer.
    pub fn signer(&self) -> Result<&SignerMiddleware<Arc<Provider<P>>, LocalWallet>> {
        self.signer.as_ref().ok_or(anyhow!("signer isn't provided"))
    }

    /// Price calculated on the basis of reserves
    pub async fn get_price(
        &self,
        factory: &Address,
        token_input: &Address,
        token_output: &Address,
    ) -> Result<Decimal> {
        let pair_addr = self
            .get_pair_address(factory, token_input, token_output)
            .await?;

        let pair = UniswapV2Pair::new(pair_addr, Arc::clone(&self.provider));

        let token_input_contract = ERC20::new(*token_input, self.provider.clone());
        let token_output_contract = ERC20::new(*token_output, self.provider.clone());

        let token_a_dec: u8 = token_input_contract.decimals().call().await?;
        let token_b_dec: u8 = token_output_contract.decimals().call().await?;

        // println!("token a dec : {}", token_a_dec);
        // println!("token b dec : {}", token_b_dec);
        // println!(
        //     "ratio : {}",
        //     f64::powi(10.0, (token_a_dec - token_b_dec) as i32)
        // );

        let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
        // println!("Decimal max value: {}", Decimal::MAX);
        // println!("Float max value: {}", f64::MAX);
        // println!("u128 max: {}", u128::MAX);
        // // let res0 = (reserve0 as f64)      
        // println!("Reserves (token_a, token_b): ({}, {})", reserve0, reserve1);
        
        // let f_res0 = reserve0.to_f64().ok_or(anyhow!("failed to convert res0 to f64"))?;
        // let f_res1 = reserve1.to_f64().ok_or(anyhow!("failed to convert res1 to f64"))?;
        
        // let f_base = f64::powi(10.0, (token_a_dec - token_b_dec) as i32);

        // let f_price = f_res1 / f_res0 * f_base;
        
        // println!("f price : {:?}", f_price);
        
        // println!("method price: {:?}", calculate_price(reserve0, reserve1, token_a_dec, token_b_dec)?);
        
        // let dec_res0 = Decimal::from(reserve0);
        // let dec_res1 = Decimal::from(reserve1);
        // let mut base = Decimal::from(10 as i32);

        // base = base.pow((token_a_dec - token_b_dec) as i64);

        // let price = (dec_res1 / dec_res0) * base;
        
        // println!("dec price: {:?}", price);
        
        Ok(calculate_price(reserve0, reserve1, token_a_dec, token_b_dec)?)
    }

    ///Price received from IUniswapRouterV2
    pub async fn get_router_price(
        &self,
        factory: &Address,
        router: &Address,
        token_input: &Address,
        token_output: &Address,
    ) -> Result<Decimal> {
        let pair_addr = self
            .get_pair_address(factory, token_input, token_output)
            .await?;

        let pair = UniswapV2Pair::new(pair_addr, Arc::clone(&self.provider));

        let router = IUniswapV2Router02::new(*router, Arc::clone(&self.provider));

        let token_input_contract = ERC20::new(*token_input, self.provider.clone());
        let token_output_contract = ERC20::new(*token_output, self.provider.clone());

        let token_input_dec: u8 = token_input_contract.decimals().call().await?;
        let token_output_dec: u8 = token_output_contract.decimals().call().await?;

        let input_amount = U256::exp10(token_input_dec.into());

        let output = router
            .get_amounts_out(input_amount, [*token_input, *token_output].to_vec())
            .call()
            .await?;
        // println!("output: {:?}", output);
        
        // let base = Decimal::from(10 as u32).pow(token_output_dec as u64);

        // let decimal_price = Decimal::from(output[1].as_u128()) / base;

        // println!("decimal_price: {:?}", output);

        Ok(calculate_price(output[0].as_u128(), output[1].as_u128(), token_input_dec, token_output_dec)?)
    }

    pub async fn get_pair_address(
        &self,
        factory: &Address,
        token_a: &Address,
        token_b: &Address,
    ) -> Result<Address> {
        let factory = UniswapFactory::new(*factory, Arc::clone(&self.provider));

        Ok(factory.get_pair(*token_a, *token_b).call().await?)
    }

    pub async fn swap_exact_eth_for_tokens(
        &self,
        router: &Address,
        amount_out_min: Decimal,
        path: &[Address],
    ) -> Result<TransactionReceipt> {
        
        if path.len() <= 1 {
            return Err(anyhow!("invalid path length"))
        }
        
        let router_contract = IUniswapV2Router02::new(*router, self.provider.clone());
        
        
        let output_token_adress= *path.iter().last().take().ok_or(anyhow!("output token adress"))?;
        let output_token_contract = ERC20::new(output_token_adress, self.provider.clone());
        
        let output_token_decimal = output_token_contract.decimals().call().await?;
        let raw_amount_out_min= {
            let base = Decimal::from(10 as u32).pow(output_token_decimal as u64);
            let dec_val = amount_out_min * base;
            let string = dec_val.to_string();
            println!("string: {}", string);
            U256::from_dec_str(&string)?
        };        
        
        
        let wallet = self.wallet()?;
        
        
        
        let out = router_contract
            .get_amounts_in(raw_amount_out_min, path.to_vec())
            .call()
            .await?;
        println!("{:?}", out);

        let gas_price = self.provider.get_gas_price().await?;
        println!("gas price: {}", gas_price);

        let mut contract_call = router_contract
            .swap_exact_eth_for_tokens(
                raw_amount_out_min,
                path.to_vec(),
                self.signer()?.address(),
                U256::from(
                    chrono::Utc::now()
                        .add(Duration::seconds(30))
                        .timestamp_millis(),
                ),
            )
            .value(U256::exp10(18).checked_div(40.into()).unwrap())
            .fill_gas_fields(self.provider())
            .await?
            .from(wallet.address());

        let estimate_gas = contract_call.estimate_gas().await?;
        println!("estimate gas: {}", estimate_gas);

        contract_call = contract_call.gas(estimate_gas);

        // println!("{:?}", contract_call);
        let tx_cost = out[0];
        println!("tx cost : {:?}", tx_cost);
        
        let encoded_tx_data = contract_call
            .calldata()
            .ok_or(anyhow!("failed to encode data"))?;
        // let nonce = self
        //     .provider
        //     .get_transaction_count(self.signer()?.address(), None)
        //     .await?;
        // println!("nonce: {}", nonce);
        let tx_req = TransactionRequest::new()
            .from(wallet.address())
            .to(*router)
            .data(encoded_tx_data)
            .value(tx_cost);

        // println!("tx_req: {:?}", tx_req);

        let pending_tx = self
            .signer()?
            .send_transaction(tx_req, Some(BlockId::Number(BlockNumber::Latest)))
            .await?;
        let hash = pending_tx.tx_hash();

        let receipt = pending_tx
            .confirmations(1)
            .await?
            .ok_or(anyhow!("receipt failed for tx: {}", hash));

        receipt
    }
}
