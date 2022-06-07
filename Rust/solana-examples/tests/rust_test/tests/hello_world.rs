use std::rc::Rc;
// #![cfg(feature = "test-bpf")]
use std::vec;

use solana_program::pubkey::Pubkey;
use solana_program::{
    system_instruction,
    system_program,
    instruction::{AccountMeta, Instruction},
};
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};

use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

use anchor_client::{Client, Cluster};
use rust_client::main::{
    hello_world
};
use hello_world::{
    entry,
    id,
    instruction::CreateAcc,
    accounts::Create
};
use anyhow::Result;
struct EnvHelloWorld {
    ctx: ProgramTestContext,
    payer: Keypair,
    anchor_client: Client,
}

impl EnvHelloWorld {
    async fn new() -> Self {
        let program_test = ProgramTest::new("hello_world", id(), processor!(entry));
        let mut ctx = program_test.start_with_context().await;
        
        let payer = Keypair::new();
        let raw_key = payer.to_bytes();

        // credit admin and user accounts
        ctx.banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &payer.pubkey(),
                    1_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash,
            ))
            .await
            .unwrap();

        let url = Cluster::Custom(
            "http://localhost:8899".to_string(),
            "ws://127.0.0.1:8900".to_string(),
        );
        
        
        
        let anchor_client = Client::new_with_options(url, Rc::new(payer), solana_sdk::commitment_config::CommitmentConfig::processed());
        // let program = anchor_client.program(id());
        // println!("client created");
        // println!("{:?}", program.rpc().get_epoch_info());     
        

        EnvHelloWorld { ctx, payer: Keypair::from_bytes(&raw_key).unwrap(), anchor_client }
        // Env{}
        
        
    }
}

#[tokio::test]
async fn test_hello_word() -> Result<()>{
    let mut env = EnvHelloWorld::new().await;
    
    println!("payer key: {}", env.payer.pubkey());
    
    
    let pda_key =
        Pubkey::find_program_address(&[b"hello".as_ref(), env.payer.pubkey().as_ref()], &hello_world::ID);

    println!("pda key: {}", pda_key.0);
    
    // let ix = Instruction::new_with_borsh(
    //     id(),
    //     &CreateAcc {},
    //     vec![
    //         AccountMeta::new(pda_key.0, false),
    //         AccountMeta::new(env.payer.pubkey(), true),
    //         AccountMeta::new(system_program::ID, false),
    //     ],
    // );
    
    
    
    // let tx = Transaction::new_signed_with_payer(
    //     &[ix],
    //     Some(&env.payer.pubkey()),
    //     &[&env.payer],
    //     env.ctx.last_blockhash,
    // );
   
    let program = env.anchor_client.program(id());
    let tx = program
    .request()
    .signer(&env.payer)
    .accounts(Create {
        greeted_account: pda_key.0,
        user: env.payer.pubkey(),
        system_program: system_program::ID,
    })
    .args(CreateAcc)
    .build_tx(env.ctx.last_blockhash)?;
    println!("tx.data: {:?}", tx.data(0));
    
    
    env.ctx.banks_client.process_transaction(tx).await.unwrap();
    
    // let client = &env.anchor_client;
    // let foo = env.ctx.banks_client.
    
    
    
    
    println!("done");
    
    Ok(())
    // hello_world(&env.anchor_client, &env.payer, id());
}

